use tracing::Level;

fn main() -> miette::Result<()> {
    tracing_subscriber::fmt()
        .compact()
        .with_max_level(Level::DEBUG)
        .init();
    let m = igloo::sandbox::Manager::new();
    let mut ctn = m.create_container("uwu".into()).unwrap();
    ctn.exec(/*ExecOptions {
        argv: env::args().skip(1).collect::<Vec<_>>(),
        mem_limit: 0,
        output_limit: 0,
        time_limit: Default::default(),
    }*/);
    ctn.wait();
    Ok(())
}
