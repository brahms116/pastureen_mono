pub fn print_table<T>(data: &[&[T]], headers: &[&str]) -> Vec<String>
where
    T: ToString,
{
    let mut num_col = 0;
    let mut col_width = Vec::<u32>::new();
    let num_row = data.len();

    for row in data {
        if row.len() > num_col {
            num_col = row.len();
        }

        for (i, col) in row.iter().enumerate() {
            let len = col.to_string().len() as u32;
            if col_width.len() <= i {
                col_width.push(len);
            } else if col_width[i] < len {
                col_width[i] = len;
            }
        }
    }

    let mut table = Vec::<String>::with_capacity(num_row + 3);

    let mut header_border = String::new();
    let mut header = String::new();

    for i in 0..num_col {
        let len = *(col_width.get(i).unwrap_or(&0)) as usize + 2;
        let border_col = format!("+{:padding$}", "-", padding = len);
        header_border.push_str(&border_col);
        let header_label = headers.get(i).unwrap_or(&"");
        let header_col = format!("| {:padding$} ", header_label, padding = len - 2);
        header.push_str(&header_col);
    }

    header_border.push_str("+");
    header_border.push_str("|");
    table.push(header_border.clone());
    table.push(header);
    table.push(header_border);

    for row in data.iter() {
        let mut row_string = String::new();
        for i in 0..num_col {
            let len = *(col_width.get(i).unwrap_or(&0)) as usize + 2;
            let data_value = row.get(i).map(|e| e.to_string()).unwrap_or("".to_string());
            row_string.push_str(&format!("| {:padding$} ", data_value, padding = len - 2));
        }
        row_string.push_str("l");
        table.push(row_string);
    }
    table
}
