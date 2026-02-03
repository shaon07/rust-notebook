// rust_ownership_borrowing_cheatsheet.rs
// Quick reference – the most important patterns you forget after 2 weeks

fn main() {
    println!("=== 1. MOVE (transfers ownership) ===");
    move_example();

    println!("\n=== 2. IMMUTABLE BORROW (&) – many readers allowed ===");
    immutable_borrow_example();

    println!("\n=== 3. MUTABLE BORROW (&mut) – one writer at a time ===");
    mutable_borrow_example();

    println!("\n=== 4. MOVE + try to use again = classic error ===");
    move_and_use_after_error();

    println!("\n=== 5. Common fix patterns ===");
    common_fix_patterns();

    println!("\n=== 6. Clone when you really need two copies ===");
    clone_example();
}

// ────────────────────────────────────────────────
// 1. MOVE – function takes ownership → original variable dies
// ────────────────────────────────────────────────
fn move_example() {
    let s1 = String::from("hello");

    takes_ownership(s1); // s1 is MOVED → invalid after this line

    // println!("{}", s1);         // error[E0382]: borrow of moved value
}

fn takes_ownership(s: String) {
    println!("took ownership: {}", s);
} // ← s dropped here → memory freed

// ────────────────────────────────────────────────
// 2. IMMUTABLE BORROW – many & at the same time is OK
// ────────────────────────────────────────────────
fn immutable_borrow_example() {
    let s = String::from("immutable example");

    print_ref(&s);
    print_ref(&s);
    print_ref(&s);
    println!("still alive: {}", s); // owner can still use it
}

fn print_ref(s: &String) {
    println!("borrowed: {}", s);
    // s.push_str("...");              // error – cannot mutate through &T
}

// ────────────────────────────────────────────────
// 3. MUTABLE BORROW – only ONE &mut at a time
// ────────────────────────────────────────────────
fn mutable_borrow_example() {
    let mut s = String::from("mutable");

    add_suffix(&mut s);
    println!("after 1st: {}", s);

    add_suffix(&mut s); // second &mut is OK – previous one ended
    println!("after 2nd: {}", s);

    // let r1 = &mut s;
    // let r2 = &mut s;                 // error – cannot have two &mut
    // r1.push_str("A");
    // r2.push_str("B");
}

fn add_suffix(s: &mut String) {
    s.push_str(" world!");
}

// ────────────────────────────────────────────────
// 4. Classic beginner error – move then use
// ────────────────────────────────────────────────
fn move_and_use_after_error() {
    let mut s = String::from("error demo");

    takes_ownership(s); // ← move happened

    // s.push_str("!!!");              // error[E0382]: use of moved value
    // println!("{}", s);
}

// ────────────────────────────────────────────────
// 5. Two most common fixes for the error above
// ────────────────────────────────────────────────
fn common_fix_patterns() {
    println!("Fix A – borrow instead of move");
    let mut s = String::from("fix A");
    mutate_via_borrow(&mut s);
    mutate_via_borrow(&mut s);
    println!("→ still works: {}", s);

    println!("\nFix B – take & give back (pass & return)");
    let mut s2 = String::from("fix B");
    s2 = mutate_and_return(s2);
    s2 = mutate_and_return(s2);
    println!("→ still works: {}", s2);
}

fn mutate_via_borrow(s: &mut String) {
    s.push_str(" (borrowed)");
}

fn mutate_and_return(mut s: String) -> String {
    s.push_str(" (returned)");
    s // ← ownership goes back to caller
}

// ────────────────────────────────────────────────
// 6. When you really need two independent values → clone
// ────────────────────────────────────────────────
fn clone_example() {
    let original = String::from("important data");

    let copy1 = original.clone();
    let copy2 = original.clone();

    println!("original: {}", original);
    println!("copy1:    {}", copy1);
    println!("copy2:    {}", copy2);

    // All three strings exist independently now
}
