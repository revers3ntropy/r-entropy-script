describe 'Performance'

perf_test_comp_cpp 1 '149995000' '
    var sum = 0;
    const n = 10000;
    var i = 0;
    for {
        if i >= n {
            break;
        };
        var j = 0;
        for {
            if j >= n {
                break;
            };
            sum = sum + 1;
            j = j + 1;
        };
        sum = sum + i;
        i = i + 1;
    };
    print_int(sum);
' '
    #include <iostream>

    int main () {
        long sum = 0;
        int n = 10000;
        int i = 0;
        while (true) {
            if (i >= n) {
                break;
            }
            int j = 0;
            while (true) {
                if (j >= n) {
                    break;
                }
                sum = sum + 1;
                j = j + 1;
            }
            sum = sum + i;
            i = i + 1;
        }
        std::cout << sum;
    }
'