use std::time::Duration;
use trpl::{Either, Html, StreamExt};

fn main() {
    let args: Vec<String> = std::env::args().collect();

    // trpl::block_on(async {
    //     let url = &args[1];
    //     match page_title(url).await {
    //         Some(title) => println!("Title: {}", title.trim()),
    //         None => println!("No title"),
    //     }
    // })
    // trpl::block_on(async {
    //     let title_fut_1 = page_title(&args[1]);
    //     let title_fut_2 = page_title(&args[2]);
    //
    //     let (url, maybe_title) = match trpl::select(title_fut_1, title_fut_2).await {
    //         Either::Left(left) => left,
    //         Either::Right(right) => right,
    //     };
    //
    //     println!("{url} returned first");
    //
    //     match maybe_title {
    //         Some(title) => println!("Its page title was: '{title}'"),
    //         None => println!("It had no title."),
    //     }
    // })

    // trpl::block_on(async {
    //     let fut1 = trpl::spawn_task(async {
    //         for i in 1..10 {
    //             println!("hi number {} from the spawned thread!", i);
    //             trpl::sleep(Duration::from_millis(500)).await;
    //         }
    //     });
    //
    //     let fut2 = async {
    //         for i in 1..5 {
    //             println!("hi number {i} from the second task!");
    //             trpl::sleep(Duration::from_millis(500)).await;
    //         }
    //     };
    //
    //     println!("{:?}", trpl::join(fut1, fut2).await);
    // });

    // trpl::block_on(async {
    //     let (tx, mut rx) = trpl::channel();
    //
    //     let tx_fut = async move {
    //         let vals = vec![
    //             String::from("hi"),
    //             String::from("from"),
    //             String::from("the"),
    //             String::from("future"),
    //         ];
    //
    //         for v in vals {
    //             tx.send(v).unwrap();
    //             trpl::sleep(Duration::from_millis(500)).await;
    //         }
    //     };
    //
    //     let rx_fut = async {
    //         while let Some(v) = rx.recv().await {
    //             println!("value: {}", v);
    //         }
    //     };
    //
    //     trpl::join(tx_fut, rx_fut).await;
    // });
    //
    // trpl::block_on(async {
    //     let slow = async {
    //         trpl::sleep(Duration::from_secs(5)).await;
    //         "Finally finished"
    //     };
    //
    //     match timeout(slow, Duration::from_secs(2)).await {
    //         Ok(message) => println!("{}", message),
    //         Err(duration) => println!("timeout: {:?}", duration),
    //     }
    // })

    trpl::block_on(async {
        let values = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
        let iter = values.iter().map(|x| x * 2);
        let mut stream = trpl::stream_from_iter(iter);

        while let Some(value) = stream.next().await {
            println!("{}", value);
        }
    })
}

async fn timeout<F: Future>(future_to_try: F, max_time: Duration) -> Result<F::Output, Duration> {
    match trpl::select(future_to_try, trpl::sleep(max_time)).await {
        Either::Left(v) => Ok(v),
        Either::Right(_) => Err(max_time),
    }
}

async fn page_title(url: &str) -> (&str, Option<String>) {
    let response_text = trpl::get(url).await.text().await;
    let title = Html::parse(&response_text)
        .select_first("title")
        .map(|title| title.inner_html());
    (url, title)
}
