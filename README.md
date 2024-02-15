# This Project is aimed at understanding and implementing the Multi Producer Single Consumer Channels in RUST 

#### There are basically two functions 

- mpsc_generic() which is used to receive data from all the child thread producers 
- mpsc_to_spmc() which is a implementation of Single Producer Multi Consumer. Where the parent thread sends function to the child thread to execute. 

#### To Run 

```bash 
cargo check 

# for mpsc 
cargo run -- mpsc

# for spmc 
cargo run -- spmc
```