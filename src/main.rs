use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use std::{error::Error, io};
use tui::{
    backend::{Backend, CrosstermBackend},
    layout::{Constraint, Direction, Layout},
    widgets::{Block, Borders, List, ListItem},
    text::{Span, Spans},
    Frame, Terminal,
};
use rand::{distributions::Alphanumeric, Rng};
use secp256k1::{
    rand::{rngs, SeedableRng},
    PublicKey, SecretKey,
};
use tiny_keccak::keccak256;
use web3::types::{Address, U256};
use web3::{
    transports::{WebSocket, Http},
    Web3,
};

struct App{
    addresses: Vec<(Address, u64)>
}

impl Default for App{
    fn default() -> App {
        App{
            addresses: load_wallet()
        }
    }
}
#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;
    let app = App::default();
    let res = run_app(&mut terminal, app).await;

    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;

    if let Err(err) = res {
        println!("{:?}", err)
    }
    

    Ok(())
}

async fn run_app<B: Backend>(terminal: &mut Terminal<B>, mut app: App) -> io::Result<()> {
    loop {
        terminal.draw(|f| ui(f,&app))?;
        let provider = create_provider("https://goerli.infura.io/v3/9aa3d95b3bc440fa88ea12eaa4456161").await;
        if let Event::Key(key) = event::read()? {
            if let KeyCode::Char('q') = key.code {
                return Ok(());
            }
            if let KeyCode::Char('a') = key.code{
                let addresses_with_amount = updated_with_amount(&provider, &mut app.addresses).await;
            }
        }
    }
}

pub async fn create_provider(url: &str) -> Web3<Http>{
    let transport = web3::transports::Http::new(url).unwrap();
    let provider = web3::Web3::new(transport); 
    return provider;
}

pub async fn updated_with_amount(provider: &Web3::<Http>, addresses: &mut Vec<(Address,u64)>){
    for el in addresses{
        let address = el.0;
        let balance = provider.eth().balance(address,None).await.unwrap();
        el.1 = balance.low_u64();
    }
}

pub fn public_key_address(public_key: &PublicKey) -> Address {
    let public_key = public_key.serialize_uncompressed();

    debug_assert_eq!(public_key[0], 0x04);
    let hash = keccak256(&public_key[1..]);

    Address::from_slice(&hash[12..])
}

fn generate_key_pair(seed: u64) -> (SecretKey, PublicKey){
    let secp = secp256k1::Secp256k1::new();
    let mut random = rngs::StdRng::seed_from_u64(seed);
    let result = secp.generate_keypair(&mut random);
    return result;
}

fn load_wallet() -> Vec<(Address, u64)>{
    let mut result: Vec<(Address, u64)> = Vec::new();
    for i in 1..10{
        
        let keyPair = generate_key_pair(i);
        let address = public_key_address(&keyPair.1);
        result.push((address, 0));
    }
    return result;
}

fn ui<B: Backend>(f: &mut Frame<B>, app: &App) {
    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints(
            [
                Constraint::Percentage(20),
                Constraint::Percentage(3),
                Constraint::Percentage(78)
            ]
            .as_ref(),
        )
        .split(f.size());

    let block = Block::default().title("Block").borders(Borders::ALL);
    f.render_widget(block, chunks[0]);
    let block = Block::default().title("Block 2").borders(Borders::ALL);
    f.render_widget(block, chunks[2]);
    let items: Vec<ListItem>= app
    .addresses
    .iter()
    .enumerate()
    .map(|(i, (m, j))| {
        let content = vec![Spans::from(Span::raw(format!("{}: {:#x} AMOUNT: {}", i, m, j)))];
        ListItem::new(content)
    })
    .collect();
    let messages =
        List::new(items).block(Block::default().borders(Borders::ALL).title("Addresses"));
    f.render_widget(messages, chunks[2]);
}