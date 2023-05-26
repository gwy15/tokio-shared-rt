use parking_lot::Mutex;
use tokio::io::AsyncWriteExt;
use tokio::net::TcpStream;
use tokio::sync::OnceCell;

static TOKIO_TCP: OnceCell<Mutex<TcpStream>> = OnceCell::const_new();

async fn init() {
    TOKIO_TCP
        .get_or_try_init(|| async {
            let s = TcpStream::connect("127.0.0.1:8000").await?;
            std::io::Result::Ok(Mutex::new(s))
        })
        .await
        .unwrap();
}
async fn write() {
    let mut tcp = TOKIO_TCP.get().unwrap().lock();
    tokio::time::sleep(std::time::Duration::from_micros(10)).await;
    tcp.write_all(&[1]).await.unwrap();
}

macro_rules! t {
    ($id:ident) => {
        #[tokio::test]
        // #[tokio_shared_rt::test(shared)]
        async fn $id() {
            init().await;
            write().await;
            eprintln!("{} done", stringify!($id));
        }
    };
}
t!(t1);
t!(t2);
t!(t3);
t!(t4);
