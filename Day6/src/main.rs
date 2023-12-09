fn race(time : i64, record : i64) ->i32 {

    let mut times_broke_record = 0;

    for buildup in 0..time
    {
        let remainingTime = time - buildup;
        let speed = buildup;

        let distance_traveled = remainingTime * speed;

        if distance_traveled > record
        {
            times_broke_record+=1;
        }
    }

    return times_broke_record;
}

fn main() {
    println!("Day6");

    let times_broke_record1 = race(  38677673 /*time*/, 234102711571236 /*record*/ );
    //let times_broke_record2 = race(  67 /*time*/, 1027 /*record*/ );
    //let times_broke_record3 = race(  76 /*time*/, 1157 /*record*/ );
    //let times_broke_record4 = race(  73 /*time*/,1236 /*record*/ );

    println!("Mutiple of values is {}", times_broke_record1);
}


