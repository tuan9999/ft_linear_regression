# ft_linear_regression

An implementation of linear regression in Rust

## Usage
### Training
To train the model please ensure that the details in `src/config/train_config.yaml` have been correctly filled out.
Then execute: `cargo run --bin train -- --config src/config/train_config.yaml`

### Testing
The theta values will be stored in the file `src/data/thetas.yaml``

To test the model please ensure that the details in `src/config/test_config.yaml` have been correctly filled out.
Then execute: `cargo run --bin predict -- --config src/config/test_config.yaml`

### Graphs
If you would like to graph the data please add the `--graph` flag.
Example: `cargo run --bin train -- --config src/config/train_config.yaml --graph`
