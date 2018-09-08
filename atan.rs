extern crate num;

use num::range_step;


fn main() {

let main_rng = 0..11;

for ind_x in main_rng {
    let mut x = (ind_x as f64)/10.0;
    
    println!("х = {}", x);    
    let rng_even = range_step(1, 40, 2);
    let mut atan = 0.0;
    let mut atan_ps = [0.0; 100];
    let one:f64 = -1.0;
    //power series    
    for (ind, p) in rng_even.enumerate() {
        let i = ind as i32;
        
        atan = atan + one.powi(i) * x.powi(p)/(p as f64);
        atan_ps[ind] = atan;
        //println!("{:?}, {:?}, {:?}", i, p, tan1);
    }

    //ration function    
    let rng = 2..20;
    
    let mut atan_rf = [0.0;100];
    let mut p = [0.0;30];
    p[0] = 1.0;
    p[1] = 3.0;
    
    let mut q = [0.0;30];
    q[0] = 1.0;
    q[1] = 3.0 + x.powi(2);

    for ind in rng {
        let i = ind as f64;

        let mut alpha = (i).powi(2);
        let beta = 2.0 * i + 1.0;
        
        let mut ax = alpha * x.powi(2);
        p[ind] = beta * p[ind - 1] + ax * p[ind - 2];
        q[ind] = beta * q[ind - 1] + ax * q[ind - 2];
        atan = p[ind]/q[ind] * x;
        atan_rf[ind-2] = atan
        //println!("i:{:<3} alpha:{:<4} beta:{:<4} atan = {}", i, alpha, beta, atan);
    }

    // Gauss
    let mut a = [0.0; 30];
    let mut b = [0.0; 30];
    
    a[0] = (1.0 + x.powi(2)).powf(-0.5);
    b[0] = 1.0;

    let mut atan_a = [0.0; 30];
    let mut atan_b = [0.0; 30];
    
    let rng = 1..20;
    for ind in rng {
        a[ind] = (a[ind-1] + b[ind-1])/2.0;
        b[ind] = (a[ind]*b[ind-1]).powf(0.5);
        let mut xx = x/(1.0 + x.powi(2)).powf(0.5);
        atan_a[ind-1] = xx/a[ind];
        atan_b[ind-1] = xx/b[ind];
        //println!("{:}, {:}, {:}, {:}", a[ind] , b[ind], atan_a, atan_b);
    }
    
    println!("{}, {}, {}, {}","power series", "ration function", "Gauss a", "Gauss b");
    let rng_arr = 0..20;
    for ind in rng_arr{
        println!("{:.10}, {:.10}, {:.10}, {:.10}", atan_ps[ind], atan_rf[ind], atan_a[ind], atan_b[ind])
    }
}


  

    
    
}
