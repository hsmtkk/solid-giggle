fn gcd(x:i32, y:i32) -> i32 {
    let start = std::cmp::min(x,y);
    for i in (1..start+1).rev() {
        if x %i==0 && y %i==0 {
            return i;
        }
    }
    1
}

trait Operation {
    fn calculate(&self, x:i32, y:i32) -> i32;
}

struct GcdiOperation {
}

impl GcdiOperation {
    fn new() -> Self {
        GcdiOperation{}
    }
}

impl Operation for GcdiOperation {
    fn calculate(&self, x:i32, y:i32) -> i32 {
        gcd(x.abs(), y.abs())
    }
}

fn oper_array(op:Box<dyn Operation>, arr:&[i32], init:i32) -> Vec<i32> {
    let mut results = Vec::new();
    let mut acc = init;
    for x in arr {
        acc = op.calculate(acc, *x);
        results.push(acc);
    }
    results
}

#[cfg(test)]
mod tests {
    use super::Operation;
    use super::oper_array;

    fn do_test(op:Box<dyn Operation>, arr:&[i32], init:i32, exp:&[i32]){
        let got = oper_array(op, arr, init);
        assert_eq!(exp, got);
    }

    #[test]
    fn test_gcdi(){
        let dta = vec![18, 69, -90, -78, 65, 40];
        let sol = vec![18, 3, 3, 3, 1, 1];
        do_test(Box::new(super::GcdiOperation::new()), &dta, dta[0], &sol);
    }
}

/*

func doTest(t *testing.T, f kata.FParam, arr []int, init int, exp []int) {
	got := kata.OperArray(f, arr, init)
	assert.Equal(t, exp, got)
}

func TestGcdi(t *testing.T) {
	var dta = []int{18, 69, -90, -78, 65, 40}
	var sol = []int{18, 3, 3, 3, 1, 1}
	doTest(t, kata.Gcdi, dta, dta[0], sol)
}

func TestSom(t *testing.T) {
	var dta = []int{357, 112, 28, -52, 644, 119}
	var sol = []int{357, 469, 497, 445, 1089, 1208}
	doTest(t, kata.Som, dta, 0, sol)
}

func TestMaxi(t *testing.T) {
	var dta = []int{10, -32, 190, 300, -42, -38, 50, 405, -46, 225, -31}
	var sol = []int{10, 10, 190, 300, 300, 300, 300, 405, 405, 405, 405}
	doTest(t, kata.Maxi, dta, dta[0], sol)
}

func TestLcmu(t *testing.T) {
	var dta = []int{6, -72, -62, -22, -23, 80}
	var sol = []int{6, 72, 2232, 24552, 564696, 5646960}
	doTest(t, kata.Lcmu, dta, dta[0], sol)
}

func TestMini(t *testing.T) {
	var dta = []int{64, -67, -43, 12, -15, 108, 12, 104, -36}
	var sol = []int{64, -67, -67, -67, -67, -67, -67, -67, -67}
	doTest(t, kata.Mini, dta, dta[0], sol)
}

*/