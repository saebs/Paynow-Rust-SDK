/********************************
Author: Sabelo Ntabeni
email: sabelo.n@yandex.com
*******************************/

/*** Rust SDK for Paynow Zimbabwe  ***/

/*
Copyright (C) 2020 by Saziwe PBC sabelo.n@yandex.com

Permission to use, copy, modify, and/or distribute this software for any purpose with or without fee is hereby granted.

THE SOFTWARE IS PROVIDED "AS IS" AND THE AUTHOR DISCLAIMS ALL WARRANTIES WITH REGARD TO THIS SOFTWARE INCLUDING ALL IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS. IN NO EVENT SHALL THE AUTHOR BE LIABLE FOR ANY SPECIAL, DIRECT, INDIRECT, OR CONSEQUENTIAL DAMAGES OR ANY DAMAGES WHATSOEVER RESULTING FROM LOSS OF USE, DATA OR PROFITS, WHETHER IN AN ACTION OF CONTRACT, NEGLIGENCE OR OTHER TORTIOUS ACTION, ARISING OUT OF OR IN CONNECTION WITH THE USE OR PERFORMANCE OF THIS SOFTWARE.
*/

/// Core Paynow functionality
pub mod core;
/// Http Responses formats or models
/// Paynow Transactions
pub mod transactions;
// pub mod responses;
pub mod types;
/// General utilities and helper code for e.g. hash creation, verification , Data Collection and formatting
mod utils;
mod trxn;

#[cfg(test)]
mod unit_tests;
