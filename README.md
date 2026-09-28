# **TWO-SIDED & N-SIDED MATCHING** #

Matching algorithms for the retrieval of all stable outcomes in two-sided and N-sided matching

## ALGORITHMS

This project comprises the following algorithms:

- N-sided matching (all stable outcomes) **NEW!**
- DeepSearch (many-to-one, one-to-one, all stable outcomes)
- BitSync (one-to-one, all stable outcomes)
- Lattice matching with hash values (one-to-one, all stable outcomes, slow mode)
- Vertex deletion mechanism (reduction via stability matrix, a speedy approach (object-oriented network) is still under construction)
- Linear programming (one-to-one, many-to-one, all outcomes for one-to-one)
- GS algorithm with GS reduced lists
- DA algorithm for many-to-many matching

Algorithms that are still under construction:
- Lattice matching with smart pointers
- Lattice matching with merging matching trees (matching tree as a partial result on Lattice layer Lk)
- N-sided matching for many-to-many-to-many-to-many matching, quotas for all agents
- BitSync for many-to-many matching
- Vertex deletion mechanism (fast version)
- Linear programming for N-sided matching
- Algorithms for presenting the full latex code of illustrations for vertex deletion mechanism

### INSTRUCTIONS

1. Please install the Rust programming language, see rustup: 
https://rustup.rs/

Run the code inside your terminal:
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

2. After Rust language is successfully installed, please download the Github folder, unpack it and open the terminal inside your folder!

3. Run the following command:
cargo run

4. For a speedy version of the program, please run the following command:
cargo build --release

This program is located in the subfolder: ./target/release/

(After successfully compiling the release version of this program, please copy the program file from ./target/release to the main folder ./ in order to preserve the folder consistency (pref,manyone_pref,pic etc.)!)

5. For trying several algorithms, please go into the src/main.rs file and uncomment/comment some test lines within the function "fn main()" by removing "//"!

This project was tested under Linux and Windows systems: Debian/Fedora and Windows 10/11
(It might be working for macOS, however, it is not tested yet.)

### ADDITIONAL REMARKS

Please note that this is an ongoing project and it will be updated regularly. If anything is not working as desired, please be patient and have a short lookup after a couple of days or a week. The code will be steadily improved from time to time.

In the upcoming days, more code will be uploaded that has not been uploaded yet, especially with respect to N-sided matching. So have a watch on this project and everything will be fine! :)

You can contact me via mwerner6@smail.uni-koeln.de

Thank you very much for your attention!
Martin Werner, University of Cologne	
