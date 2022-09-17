fn generate(num_rows: i32) -> Vec<Vec<i32>> {
    let mut triangle: Vec<Vec<i32>> = Vec::new();

    if num_rows < 1 {
        return triangle;
    }

    let mut curr_row: Vec<i32> = vec![1];
    for _ in 0..num_rows{
        let mut next_row: Vec<i32> = vec![1; curr_row.len() + 1];
        for i in 1..curr_row.len(){
            next_row[i] = curr_row[i-1] + curr_row[i];
        }
        triangle.push(curr_row);
        curr_row = next_row;
    }
    return triangle;
}





fn main() {
    generate(5);
}
