# Class*y* Group

The Group of unknown order that is used to sample our Class groups of imaginary quadratic order

Class groups of binary quadratic forms omits the trusted setup that RSA needs.
The order of the class group of a negative prime discriminant d, where |d| ≡ 3 mod 4, 
is believed to be difficult to compute when |d| is sufficiently large, making the order 
of the class group effectively unknown. Therefore, a suitable discriminant — and its associated 
class group — can be chosen without the need for a trusted setup, which is a major advantage for 
using class groups in applications requiring groups of unknown order.


group_class_op          time:   [1.7747 us 1.7954 us 1.8216 us]                            
Found 5 outliers among 100 measurements (5.00%)
  3 (3.00%) high mild
  2 (2.00%) high severe

group_class_exp         time:   [15.983 ms 16.357 ms 16.712 ms]                             
Found 3 outliers among 100 measurements (3.00%)
  2 (2.00%) high mild
  1 (1.00%) high severe

group_class_square      time:   [762.11 ns 775.93 ns 791.58 ns]                                
Found 11 outliers among 100 measurements (11.00%)
  7 (7.00%) high mild
  4 (4.00%) high severe

#

group_class_op          time:   [1.5665 us 1.5763 us 1.5880 us]                            
Found 6 outliers among 100 measurements (6.00%)
  2 (2.00%) high mild
  4 (4.00%) high severe

Benchmarking group_class_exp: Warming up for 3.0000 s
Warning: Unable to complete 100 samples in 5.0s. You may wish to increase target time to 70.6s or reduce sample count to 10
group_class_exp         time:   [13.789 ms 13.830 ms 13.875 ms]                             
Found 9 outliers among 100 measurements (9.00%)
  3 (3.00%) high mild
  6 (6.00%) high severe

group_class_square      time:   [578.73 ns 582.13 ns 586.30 ns]                                
Found 15 outliers among 100 measurements (15.00%)
  3 (3.00%) low mild
  5 (5.00%) high mild
  7 (7.00%) high severe

blake2                  time:   [121.22 ns 121.72 ns 122.33 ns]                   
Found 11 outliers among 100 measurements (11.00%)
  1 (1.00%) low severe
  3 (3.00%) low mild
  4 (4.00%) high mild
  3 (3.00%) high severe

hash_to_prime           time:   [303.36 us 306.26 us 309.21 us]                          
Found 1 outliers among 100 measurements (1.00%)
  1 (1.00%) high severe

- https://github.com/Chia-Network/vdf-competition/blob/master/classgroups.pdf