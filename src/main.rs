/********************************************************************/
/* Load Libraries                                                   */
/********************************************************************/
use binance::websockets::*;
use std::sync::atomic::{AtomicBool};
use mysql::*;
use mysql::prelude::*;


fn main() -> std::result::Result<(), Box<dyn std::error::Error>> {
    let keep_running = AtomicBool::new(true);
    let agg_trade: String = format!("!ticker@arr");
    let url = "mysql://root:example@localhost:3306/test";
    let pool = Pool::new(url)?;
    let mut conn = pool.get_conn()?;

    let mut web_socket: WebSockets = WebSockets::new(|event: WebsocketEvent| {
    	match event {
    	    WebsocketEvent::DayTickerAll(ticker_events) => {
    	        for tick_event in ticker_events {
                    /********************************************************************/
                    /* Get current price                                                */
                    /********************************************************************/
        		    if tick_event.symbol == "BTCUSDT" || tick_event.symbol == "BNBUSDT" || tick_event.symbol == "ETHUSDT" {
            			let current_price: f32 = tick_event.current_close.parse().unwrap();
                        "INSERT INTO prices (symbol, price) VALUES (?, ?)"
                            .with((tick_event.symbol, current_price))
                            .run(&mut conn);
            			println!("price {}", current_price);
    		        }
    		    }
    	    },
    	    _ => (),
        };

        Ok(())
    });

    web_socket.connect(&agg_trade).unwrap();
    web_socket.event_loop(&keep_running)?;
    Ok(())
}
