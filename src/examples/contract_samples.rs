//! Examples of populating fields that define various types of contacts

#![allow(clippy::field_reassign_with_default)]

use crate::core::contract::{ComboLeg, Contract, PositionType};

//==================================================================================================
pub fn eur_gbp_fx() -> Contract {
    let mut contract = Contract::default();
    contract.symbol = "EUR".to_string();
    contract.sec_type = "CASH".to_string();
    contract.currency = "GBP".to_string();
    contract.exchange = "IDEALPRO".to_string();

    contract
}

//==================================================================================================
pub fn index() -> Contract {
    let mut contract = Contract::default();
    contract.symbol = "DAX".to_string();
    contract.sec_type = "IND".to_string();
    contract.currency = "EUR".to_string();
    contract.exchange = "DTB".to_string();

    contract
}

//==================================================================================================
pub fn cfd() -> Contract {
    let mut contract = Contract::default();
    contract.symbol = "IBDE30".to_string();
    contract.sec_type = "cfd".to_string();
    contract.currency = "EUR".to_string();
    contract.exchange = "SMART".to_string();

    contract
}

//==================================================================================================
pub fn european_stock() -> Contract {
    let mut contract = Contract::default();
    contract.symbol = "BMW".to_string();
    contract.sec_type = "STK".to_string();
    contract.currency = "EUR".to_string();
    contract.exchange = "SMART".to_string();
    contract.primary_exchange = "IBIS".to_string();
    contract
}

//==================================================================================================
pub fn european_stock2() -> Contract {
    let mut contract = Contract::default();
    contract.symbol = "NOKIA".to_string();
    contract.sec_type = "STK".to_string();
    contract.currency = "EUR".to_string();
    contract.exchange = "SMART".to_string();
    contract.primary_exchange = "HEX".to_string();
    contract
}

//==================================================================================================
pub fn option_at_ise() -> Contract {
    let mut contract = Contract::default();
    contract.symbol = "COF".to_string();
    contract.sec_type = "OPT".to_string();
    contract.currency = "USD".to_string();
    contract.exchange = "ISE".to_string();
    contract.last_trade_date_or_contract_month = "20190315".to_string();
    contract.right = "P".to_string();
    contract.strike = 105.0;
    contract.multiplier = "100".to_string();
    contract
}

//==================================================================================================
pub fn bond_with_cusip() -> Contract {
    let mut contract = Contract::default();
    // enter CUSIP as symbol
    contract.symbol = "912828C57".to_string();
    contract.sec_type = "BOND".to_string();
    contract.exchange = "SMART".to_string();
    contract.currency = "USD".to_string();

    contract
}

//==================================================================================================
pub fn bond() -> Contract {
    let mut contract = Contract::default();
    contract.con_id = 15960357;
    contract.exchange = "SMART".to_string();

    contract
}

//==================================================================================================
pub fn mutual_fund() -> Contract {
    let mut contract = Contract::default();
    contract.symbol = "VINIX".to_string();
    contract.sec_type = "FUND".to_string();
    contract.exchange = "FUNDSERV".to_string();
    contract.currency = "USD".to_string();

    contract
}

//==================================================================================================
pub fn commodity() -> Contract {
    let mut contract = Contract::default();
    contract.symbol = "XAUUSD".to_string();
    contract.sec_type = "CMDTY".to_string();
    contract.exchange = "SMART".to_string();
    contract.currency = "USD".to_string();

    contract
}

//==================================================================================================
pub fn usstock() -> Contract {
    let mut contract = Contract::default();
    contract.symbol = "AMZN".to_string();
    contract.sec_type = "STK".to_string();
    contract.currency = "USD".to_string();
    //In the API side, NASDAQ is always defined as ISLAND in the exchange field
    contract.exchange = "ISLAND".to_string();
    //stkcontract]
    contract
}

//==================================================================================================
pub fn usstock_with_primary_exch() -> Contract {
    let mut contract = Contract::default();
    contract.symbol = "MSFT".to_string();
    contract.sec_type = "STK".to_string();
    contract.currency = "USD".to_string();
    contract.exchange = "SMART".to_string();
    //Specify the Primary Exchange attribute to avoid contract ambiguity
    //(there is an ambiguity because there is also a MSFT contract with primary exchange = "AEB")
    contract.primary_exchange = "ISLAND".to_string();
    //stkcontractwithprimary]
    contract
}

//==================================================================================================
pub fn us_stock_at_smart() -> Contract {
    let mut contract = Contract::default();
    contract.symbol = "MSFT".to_string();
    contract.sec_type = "STK".to_string();
    contract.currency = "USD".to_string();
    contract.exchange = "SMART".to_string();
    contract
}

//==================================================================================================
pub fn us_option_contract() -> Contract {
    let mut contract = Contract::default();
    contract.symbol = "GOOG".to_string();
    contract.sec_type = "OPT".to_string();
    contract.exchange = "SMART".to_string();
    contract.currency = "USD".to_string();
    contract.last_trade_date_or_contract_month = "20201218".to_string();
    contract.strike = 1180.0;
    contract.right = "C".to_string();
    contract.multiplier = "100".to_string();

    contract
}

//==================================================================================================
pub fn option_at_box() -> Contract {
    let mut contract = Contract::default();
    contract.symbol = "GOOG".to_string();
    contract.sec_type = "OPT".to_string();
    contract.exchange = "BOX".to_string();
    contract.currency = "USD".to_string();
    contract.last_trade_date_or_contract_month = "20201218".to_string();
    contract.strike = 1180.0;
    contract.right = "C".to_string();
    contract.multiplier = "100".to_string();

    contract
}

//==================================================================================================
/// Option contracts require far more information since there are many
/// contracts having the exact same attributes such as symbol, currency,
/// strike, etc. This can be overcome by adding more details such as the
//' trading class
pub fn option_with_trading_class() -> Contract {
    let mut contract = Contract::default();
    contract.symbol = "SANT".to_string();
    contract.sec_type = "OPT".to_string();
    contract.exchange = "MEFFRV".to_string();
    contract.currency = "EUR".to_string();
    contract.last_trade_date_or_contract_month = "20190621".to_string();
    contract.strike = 7.5;
    contract.right = "C".to_string();
    contract.multiplier = "100".to_string();
    contract.trading_class = "SANEU".to_string();

    contract
}

//==================================================================================================
/// Using the contract's own symbol (local_symbol) can greatly simplify a
/// contract description. Watch out for the spaces within the local symbol!
pub fn option_with_local_symbol() -> Contract {
    let mut contract = Contract::default();
    //Watch out for the spaces within the local symbol!
    contract.local_symbol = "C DBK  DEC 20  1600".to_string();
    contract.sec_type = "OPT".to_string();
    contract.exchange = "DTB".to_string();
    contract.currency = "EUR".to_string();

    contract
}

//==================================================================================================
/// Dutch Warrants (IOPTs) can be defined using the local symbol or conid
pub fn dutch_warrant() -> Contract {
    let mut contract = Contract::default();
    contract.local_symbol = "B881G".to_string();
    contract.sec_type = "IOPT".to_string();
    contract.exchange = "SBF".to_string();
    contract.currency = "EUR".to_string();

    contract
}

//==================================================================================================
/// Future contracts also require an expiration date but are less
/// complicated than options.
pub fn simple_future() -> Contract {
    let mut contract = Contract::default();
    contract.symbol = "ES".to_string();
    contract.sec_type = "FUT".to_string();
    contract.exchange = "GLOBEX".to_string();
    contract.currency = "USD".to_string();
    contract.last_trade_date_or_contract_month = "202009".to_string();

    contract
}

//==================================================================================================
/// Rather than giving expiration dates we can also provide the local symbol
/// attributes such as symbol, currency, strike, etc.
pub fn future_with_local_symbol() -> Contract {
    let mut contract = Contract::default();
    contract.sec_type = "FUT".to_string();
    contract.exchange = "GLOBEX".to_string();
    contract.currency = "USD".to_string();
    contract.local_symbol = "ESU0".to_string();

    contract
}

//==================================================================================================
pub fn future_with_multiplier() -> Contract {
    let mut contract = Contract::default();
    contract.symbol = "DAX".to_string();
    contract.sec_type = "FUT".to_string();
    contract.exchange = "DTB".to_string();
    contract.currency = "EUR".to_string();
    contract.last_trade_date_or_contract_month = "201903".to_string();
    contract.multiplier = "5".to_string();

    contract
}

//==================================================================================================
/// Note the space in the symbol!
pub fn wrong_contract() -> Contract {
    let mut contract = Contract::default();
    contract.symbol = " IJR ".to_string();
    contract.con_id = 9579976;
    contract.sec_type = "STK".to_string();
    contract.exchange = "SMART".to_string();
    contract.currency = "USD".to_string();
    contract
}

//==================================================================================================
pub fn futures_on_options() -> Contract {
    let mut contract = Contract::default();
    contract.symbol = "ES".to_string();
    contract.sec_type = "FOP".to_string();
    contract.exchange = "GLOBEX".to_string();
    contract.currency = "USD".to_string();
    contract.last_trade_date_or_contract_month = "20190315".to_string();
    contract.strike = 2900.0;
    contract.right = "C".to_string();
    contract.multiplier = "50".to_string();

    contract
}

//==================================================================================================
/// It is also possible to deine contracts based on their ISIN (IBKR STK
/// sample).
pub fn by_isin() -> Contract {
    let mut contract = Contract::default();
    contract.sec_id_type = "ISIN".to_string();
    contract.sec_id = "US45841N1072".to_string();
    contract.exchange = "SMART".to_string();
    contract.currency = "USD".to_string();
    contract.sec_type = "STK".to_string();
    contract
}

//==================================================================================================
/// Or their con_id (EUR.uSD sample).
/// Note: passing a contract containing the con_id can cause problems if one of
/// the other provided attributes does not match 100% with what is in IB's
/// database. This is particularly important for contracts such as Bonds which
/// may change their description from one day to another.
/// If the con_id is provided, it is best not to give too much information as
/// in the example below.
pub fn by_con_id() -> Contract {
    let mut contract = Contract::default();
    contract.sec_type = "CASH".to_string();
    contract.con_id = 12087792;
    contract.exchange = "IDEALPRO".to_string();
    contract
}

//==================================================================================================
/// Ambiguous contracts are great to use with
/// Contract::req_contract_details. This way
/// you can query the whole option chain for an underlying. Bear in mind that
/// there are pacing mechanisms in place which will delay any further responses
/// from the TWS to prevent abuse.
pub fn option_for_query() -> Contract {
    let mut contract = Contract::default();
    contract.symbol = "FISV".to_string();
    contract.sec_type = "OPT".to_string();
    contract.exchange = "SMART".to_string();
    contract.currency = "USD".to_string();

    contract
}

//==================================================================================================
pub fn option_combo_contract() -> Contract {
    let mut contract = Contract::default();
    contract.symbol = "DBK".to_string();
    contract.sec_type = "BAG".to_string();
    contract.currency = "EUR".to_string();
    contract.exchange = "DTB".to_string();

    let mut leg1 = ComboLeg::default();
    leg1.con_id = 317960956; //DBK JUN 21 2019 C
    leg1.ratio = 1.0;
    leg1.action = "BUY".to_string();
    leg1.exchange = "DTB".to_string();

    let mut leg2 = ComboLeg::default();
    leg2.con_id = 334216780; //DBK MAR 15 2019 C
    leg2.ratio = 1.0;
    leg2.action = "SELL".to_string();
    leg2.exchange = "DTB".to_string();

    contract.combo_legs = vec![];
    contract.combo_legs.push(leg1);
    contract.combo_legs.push(leg2);
    //bagoptcontract]
    contract
}

//==================================================================================================
/// STK Combo contract
/// Leg 1: 43645865 - IBKR's STK
/// Leg 2: 9408 - McDonald's STK
pub fn stock_combo_contract() -> Contract {
    let mut contract = Contract::default();
    contract.symbol = "IBKR,MCD".to_string();
    contract.sec_type = "BAG".to_string();
    contract.currency = "USD".to_string();
    contract.exchange = "SMART".to_string();

    let mut leg1 = ComboLeg::default();
    leg1.con_id = 43645865; //IBKR STK
    leg1.ratio = 1.0;
    leg1.action = "BUY".to_string();
    leg1.exchange = "SMART".to_string();

    let mut leg2 = ComboLeg::default();
    leg2.con_id = 9408; //MCD STK
    leg2.ratio = 1.0;
    leg2.action = "SELL".to_string();
    leg2.exchange = "SMART".to_string();

    contract.combo_legs = vec![];
    contract.combo_legs.push(leg1);
    contract.combo_legs.push(leg2);

    contract
}

//==================================================================================================
/// CBOE volatility Index Future combo contract
pub fn future_combo_contract() -> Contract {
    let mut contract = Contract::default();
    contract.symbol = "VIX".to_string();
    contract.sec_type = "BAG".to_string();
    contract.currency = "USD".to_string();
    contract.exchange = "CFE".to_string();

    let mut leg1 = ComboLeg::default();
    leg1.con_id = 438391466; // VIX FUT 201903
    leg1.ratio = 1.0;
    leg1.action = "BUY".to_string();
    leg1.exchange = "CFE".to_string();
    leg1.exempt_code = -1;
    leg1.open_close = PositionType::SamePos;

    let mut leg2 = ComboLeg::default();
    leg2.con_id = 394987014; // VIX FUT 201904
    leg2.ratio = 1.0;
    leg2.action = "SELL".to_string();
    leg2.exchange = "CFE".to_string();
    leg2.exempt_code = -1;
    leg2.open_close = PositionType::SamePos;

    contract.combo_legs = vec![];
    contract.combo_legs.push(leg1);
    contract.combo_legs.push(leg2);

    contract
}

//==================================================================================================
pub fn smart_future_combo_contract() -> Contract {
    let mut contract = Contract::default();
    contract.symbol = "WTI".to_string(); // WTI,COIL spread. Symbol can be defined as first leg symbol ("WTI") or currency ("USD")
    contract.sec_type = "BAG".to_string();
    contract.currency = "USD".to_string();
    contract.exchange = "SMART".to_string();

    let mut leg1 = ComboLeg::default();
    leg1.con_id = 55928698; // WTI future June 2017
    leg1.ratio = 1.0;
    leg1.action = "BUY".to_string();
    leg1.exchange = "IPE".to_string();

    let mut leg2 = ComboLeg::default();
    leg2.con_id = 55850663; // COIL future June 2017
    leg2.ratio = 1.0;
    leg2.action = "SELL".to_string();
    leg2.exchange = "IPE".to_string();

    contract.combo_legs = vec![];
    contract.combo_legs.push(leg1);
    contract.combo_legs.push(leg2);

    contract
}

//==================================================================================================
pub fn inter_cmdty_futures_contract() -> Contract {
    let mut contract = Contract::default();
    contract.symbol = "CL.BZ".to_string(); //symbol is 'local symbol' of intercommodity spread.
    contract.sec_type = "BAG".to_string();
    contract.currency = "USD".to_string();
    contract.exchange = "NYMEX".to_string();

    let mut leg1 = ComboLeg::default();
    leg1.con_id = 47207310; //CL Dec'16 @NYMEX
    leg1.ratio = 1.0;
    leg1.action = "BUY".to_string();
    leg1.exchange = "NYMEX".to_string();

    let mut leg2 = ComboLeg::default();
    leg2.con_id = 47195961; //BZ Dec'16 @NYMEX
    leg2.ratio = 1.0;
    leg2.action = "SELL".to_string();
    leg2.exchange = "NYMEX".to_string();

    contract.combo_legs = vec![];
    contract.combo_legs.push(leg1);
    contract.combo_legs.push(leg2);

    contract
}

//==================================================================================================
pub fn news_feed_for_query() -> Contract {
    let mut contract = Contract::default();
    contract.sec_type = "NEWS".to_string();
    contract.exchange = "BRFG".to_string(); //Briefing Trader

    contract
}

//==================================================================================================
pub fn brfgbroadtape_news_feed() -> Contract {
    let mut contract = Contract::default();
    contract.symbol = "BRFG:BRFG_ALL".to_string();
    contract.sec_type = "NEWS".to_string();
    contract.exchange = "BRFG".to_string();

    contract
}

//==================================================================================================
pub fn djnlbroadtape_news_feed() -> Contract {
    let mut contract = Contract::default();
    contract.symbol = "DJNL:DJNL_ALL".to_string();
    contract.sec_type = "NEWS".to_string();
    contract.exchange = "DJNL".to_string();

    contract
}

//==================================================================================================
pub fn djtopbroadtape_news_feed() -> Contract {
    let mut contract = Contract::default();
    contract.symbol = "DJTOP:ASIAPAC".to_string();
    contract.sec_type = "NEWS".to_string();
    contract.exchange = "DJTOP".to_string();

    contract
}

//==================================================================================================
pub fn brfupdnbroadtape_news_feed() -> Contract {
    let mut contract = Contract::default();
    contract.symbol = "BRFUPDN:BRF_ALL".to_string();
    contract.sec_type = "NEWS".to_string();
    contract.exchange = "BRFUPDN".to_string();

    contract
}

//==================================================================================================
pub fn cont_fut() -> Contract {
    let mut contract = Contract::default();
    contract.symbol = "ES".to_string();
    contract.sec_type = "CONTFUT".to_string();
    contract.exchange = "GLOBEX".to_string();

    contract
}

//==================================================================================================
pub fn cont_and_expiring_fut() -> Contract {
    let mut contract = Contract::default();
    contract.symbol = "ES".to_string();
    contract.sec_type = "FUT+CONTFUT".to_string();
    contract.exchange = "GLOBEX".to_string();

    contract
}

//==================================================================================================
pub fn jefferies_contract() -> Contract {
    let mut contract = Contract::default();
    contract.symbol = "AAPL".to_string();
    contract.sec_type = "STK".to_string();
    contract.exchange = "JEFFALGO".to_string();
    contract.currency = "USD".to_string();

    contract
}

//==================================================================================================
pub fn csfbcontract() -> Contract {
    let mut contract = Contract::default();
    contract.symbol = "IBKR".to_string();
    contract.sec_type = "STK".to_string();
    contract.exchange = "CSFBALGO".to_string();
    contract.currency = "USD".to_string();

    contract
}

//==================================================================================================
pub fn usstock_cfd() -> Contract {
    let mut contract = Contract::default();
    contract.symbol = "IBM".to_string();
    contract.sec_type = "cfd".to_string();
    contract.currency = "USD".to_string();
    contract.exchange = "SMART".to_string();

    contract
}

//==================================================================================================
pub fn european_stock_cfd() -> Contract {
    let mut contract = Contract::default();
    contract.symbol = "BMW".to_string();
    contract.sec_type = "cfd".to_string();
    contract.currency = "EUR".to_string();
    contract.exchange = "SMART".to_string();

    contract
}

//==================================================================================================
pub fn cash_cfd() -> Contract {
    let mut contract = Contract::default();
    contract.symbol = "EUR".to_string();
    contract.sec_type = "cfd".to_string();
    contract.currency = "USD".to_string();
    contract.exchange = "SMART".to_string();

    contract
}

//==================================================================================================
pub fn qbalgo_contract() -> Contract {
    let mut contract = Contract::default();
    contract.symbol = "ES".to_string();
    contract.sec_type = "FUT".to_string();
    contract.exchange = "QBALGO".to_string();
    contract.currency = "USD".to_string();
    contract.last_trade_date_or_contract_month = "202009".to_string();

    contract
}
