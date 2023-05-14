pub fn print_table<T, const K: usize>(data: &[[T; K]], headers: &[&str; K]) -> Vec<String>
where
    T: ToString,
{
    let mut col_width = [0; K];
    let num_row = data.len();

    for row in data {
        for (i, col) in row.iter().enumerate() {
            let len = col.to_string().len() as u32;
            if col_width[i] < len {
                col_width[i] = len;
            }
        }
    }

    for (i, header) in headers.iter().enumerate() {
        let len = header.len() as u32;
        let current_col_width = col_width[i];
        if current_col_width < len {
            col_width[i] = len;
        }
    }

    let mut table = Vec::<String>::with_capacity(num_row + 4);

    let num_col = headers.len();

    let column_width_sum = col_width.iter().sum::<u32>() as usize;
    let padding = num_col * 2;
    let borders = num_col + 1;
    let table_width = column_width_sum + padding + borders;

    let mut header_border = String::with_capacity(table_width);
    let mut header = String::with_capacity(table_width);

    for i in 0..K {
        let len = *(col_width.get(i).unwrap_or(&0)) as usize + 2;
        let border_col = format!("+{:-<padding$}", "", padding = len);
        header_border.push_str(&border_col);
        let header_label = headers.get(i).unwrap_or(&"");
        let header_col = format!("| {:padding$} ", header_label, padding = len - 2);
        header.push_str(&header_col);
    }

    header_border.push_str("+");
    header.push_str("|");
    table.push(header_border.clone());
    table.push(header);
    table.push(header_border.clone());

    for row in data.iter() {
        let mut row_string = String::with_capacity(table_width);
        for i in 0..num_col {
            let len = *(col_width.get(i).unwrap_or(&0)) as usize + 2;
            let data_value = row.get(i).map(|e| e.to_string()).unwrap_or("".to_string());
            row_string.push_str(&format!("| {:padding$} ", data_value, padding = len - 2));
        }
        row_string.push_str("|");
        table.push(row_string);
    }
    table.push(header_border);
    table
}

#[cfg(test)]
#[test]
fn test_table() {
    let sample_data: Vec<[&str; 3]> = vec![
        ["Abel", "34", "male"],
        ["Tong tong wowwwww such long length", "34", "female"],
        ["Casper", "25", "male"],
    ];

    let headers = ["Name", "Points", "Gender"];
    let table = print_table(&sample_data, &headers);

    // Adds new line at the end, because sample output contains a new line at the end of the file
    let generated_output = format!("{}\n", table.join("\n"));

    let sample_output = include_str!("sample_output");
    assert_eq!(sample_output, generated_output);
}
