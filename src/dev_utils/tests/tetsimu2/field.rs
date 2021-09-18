use crate::tetsimu2::core::FieldCellValue;
use crate::tetsimu2::core::MAX_FIELD_WIDTH;
use crate::tetsimu2::field::Field;

pub fn make_field(field_data: &str) -> Field {
  if field_data.len() % MAX_FIELD_WIDTH as usize != 0 {
    panic!("Field_data length must be multiple of {}.", MAX_FIELD_WIDTH);
  }

  let mut rest_of_field_data = String::from(field_data);
  let mut field = Field::new();

  let mut rest_of_field_data_len = rest_of_field_data.len();
  let mut y = 0;
  while rest_of_field_data_len > 0 {
    let row_data = rest_of_field_data.split_off(rest_of_field_data_len - MAX_FIELD_WIDTH as usize);

    let mut x = 0;
    for t in row_data.chars() {
      let cell_value = match t {
        'N' => FieldCellValue::None,
        'I' => FieldCellValue::I,
        'J' => FieldCellValue::J,
        'L' => FieldCellValue::L,
        'O' => FieldCellValue::O,
        'S' => FieldCellValue::S,
        'T' => FieldCellValue::T,
        'Z' => FieldCellValue::Z,
        'G' => FieldCellValue::Garbage,
        _ => panic!("Cannot convert '{}' into FieldCellValue", t),
      };

      field.set_cell(x, y, cell_value);
      x += 1;
    }

    rest_of_field_data_len -= MAX_FIELD_WIDTH as usize;
    y += 1;
  }

  field
}
