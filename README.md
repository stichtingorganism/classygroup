# Class*y* Group


https://github.com/Chia-Network/vdf-competition/blob/master/classgroups.pdf

The Group of unknown order that is used to sample our Class groups of imaginary quadratic order
Binary quadratic forms 
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
