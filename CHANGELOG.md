
Changelog
=========

0.2.1
-----
*2020-03-12*

Features:
- Message-passing to organize the code
- Added .gitignore and Cargo.lock

Notes:
- Instead of passing state/gui/config around with (A)Rc and RefCell, there is now a single source of App, mutably borrowed into the message receiever. The messages are currently handled by excessive use of associated functions with `&mut App` as the first argument, but this could (and probably should) be changed to something more idiomatic rust. Perhaps traits.



0.2.0
-----
*2019-10-12*


Features:
- Major restructuring.
- Feed-dialog now disappears on focus away.
- Changed from `serde` to `xfce-rc` for storing configuration 

Fixes: 
- Restarting polling after configuration.
- Linking properly, 


0.1.0
-----
*2019-10-06*

Features:
- Initial proof-of-concept