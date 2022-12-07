describe 'Integer GT'

expect_expr_bool 'false' '1 > 2'
expect_expr_bool 'true' '2 > 1'
expect_expr_bool 'false' '1 > 1'
expect_expr_bool 'true' '1 > 0'
expect_expr_bool 'false' '0 > 1'
expect_expr_bool 'true' '1 > -1'
expect_expr_bool 'false' '-1 > 1'
expect_expr_bool 'true' '1 + 4 > 2'
expect_expr_bool 'false' '1 + 4 > 2 * 3'
expect_err 'TypeError' 'true > 2'
expect_err 'TypeError' '1 > 2 > 4'
expect_err 'TypeError' '2 > ""'
expect_err 'TypeError' '"" > ""'
expect_err 'TypeError' '"" > true'
expect_err 'TypeError' '2 > false'
expect_err 'TypeError' '"" > 2'
expect_err 'TypeError' 'fn a(); a > 4'


describe 'Integer LT'

expect_expr_bool 'true' '1 < 2'
expect_expr_bool 'false' '2 < 1'
expect_expr_bool 'false' '1 < 1'
expect_expr_bool 'false' '1 < 0'
expect_expr_bool 'true' '0 < 1'
expect_expr_bool 'false' '1 < -1'
expect_expr_bool 'true' '-1 < 1'
expect_expr_bool 'false' '1 + 4 < 2'
expect_expr_bool 'true' '1 + 4 < 2 * 3'
expect_err 'TypeError' 'true < 2'
expect_err 'TypeError' '1 < 2 < 4'
expect_err 'TypeError' '2 < ""'
expect_err 'TypeError' '"" < ""'
expect_err 'TypeError' '"" < true'
expect_err 'TypeError' '2 < false'
expect_err 'TypeError' '"" < 2'
expect_err 'TypeError' 'fn a(); a < 4'


describe 'Integer GE'

expect_expr_bool 'false' '1 >= 2'
expect_expr_bool 'true' '2 >= 1'
expect_expr_bool 'true' '1 >= 1'
expect_expr_bool 'true' '1 >= 0'
expect_expr_bool 'false' '0 >= 1'
expect_expr_bool 'true' '1 >= -1'
expect_expr_bool 'false' '-1 >= 1'
expect_expr_bool 'true' '1 + 4 >= 2'
expect_expr_bool 'false' '1 + 4 >= 2 * 3'
expect_err 'TypeError' 'true >= 2'
expect_err 'TypeError' '1 >= 2 >= 4'
expect_err 'TypeError' '2 >= ""'
expect_err 'TypeError' '"" >= ""'
expect_err 'TypeError' '"" >= true'
expect_err 'TypeError' '2 >= false'
expect_err 'TypeError' '"" >= 2'
expect_err 'TypeError' 'fn a(); a >= 4'


describe 'Integer LE'

expect_expr_bool 'true' '1 <= 2'
expect_expr_bool 'false' '2 <= 1'
expect_expr_bool 'true' '1 <= 1'
expect_expr_bool 'false' '1 <= 0'
expect_expr_bool 'true' '0 <= 1'
expect_expr_bool 'false' '1 <= -1'
expect_expr_bool 'true' '-1 <= 1'
expect_expr_bool 'false' '1 + 4 <= 2'
expect_expr_bool 'true' '1 + 4 <= 2 * 3'
expect_err 'TypeError' 'true <= 2'
expect_err 'TypeError' '1 <= 2 <= 4'
expect_err 'TypeError' '2 <= ""'
expect_err 'TypeError' '"" <= ""'
expect_err 'TypeError' '"" <= true'
expect_err 'TypeError' '2 <= false'
expect_err 'TypeError' '"" <= 2'
expect_err 'TypeError' 'fn a(); a <= 4'


describe 'Integer EQ'

expect_expr_bool 'false' '1 == 2'
expect_expr_bool 'false' '2 == 1'
expect_expr_bool 'true' '1 == 1'
expect_expr_bool 'false' '1 == 0'
expect_expr_bool 'false' '0 == 1'
expect_expr_bool 'false' '1 == -1'
expect_expr_bool 'false' '-1 == 1'
expect_expr_bool 'false' '1 + 4 == 2'
expect_expr_bool 'false' '1 + 4 == 2 * 3'
expect_err 'TypeError' 'true == 2'
expect_err 'TypeError' '1 == 2 == 4'
expect_err 'TypeError' '2 == ""'
expect_err 'TypeError' '"" == ""'
expect_err 'TypeError' '"" == true'
expect_err 'TypeError' '2 == false'
expect_err 'TypeError' 'false == false'
expect_err 'TypeError' '"" == 2'
expect_err 'TypeError' 'fn a(); a == 4'


describe 'Integer NE'

expect_expr_bool 'true' '1 != 2'
expect_expr_bool 'true' '2 != 1'
expect_expr_bool 'false' '1 != 1'
expect_expr_bool 'true' '1 != 0'
expect_expr_bool 'true' '0 != 1'
expect_expr_bool 'true' '1 != -1'
expect_expr_bool 'true' '-1 != 1'
expect_expr_bool 'true' '1 + 4 != 2'
expect_expr_bool 'true' '1 + 4 != 2 * 3'
expect_err 'TypeError' 'true != 2'
expect_err 'TypeError' '1 != 2 != 4'
expect_err 'TypeError' '2 != ""'
expect_err 'TypeError' '"" != ""'
expect_err 'TypeError' '"" != true'
expect_err 'TypeError' '2 != false'
expect_err 'TypeError' 'false != false'
expect_err 'TypeError' '"" != 2'
expect_err 'TypeError' 'fn a(); a != 4'