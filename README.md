# processcsv
This code reads data from a CSV file named "data.csv" and deserializes it into a Rust struct named Record. The csv crate's ReaderBuilder struct is used to construct a Reader object with the following configuration:

has_headers(true) tells the reader that the CSV file has a header row, which will be used to name the fields of the Record struct.
from_path("data.csv") tells the reader to read from the "data.csv" file.
The code then loops over each row of the CSV file using the deserialize method, which attempts to deserialize the row into a Record struct. If deserialization is successful, the resulting Record is printed to the console using println!. If an error occurs during deserialization, the error message is printed to the console using eprintln!.

Note that the Record struct has four fields: id, name, email, and age, all of which have different data types. The #[derive] attribute above the struct declaration tells the Rust compiler to automatically generate implementations of the Debug, Serialize, and Deserialize traits for the struct, which are used to print the struct to the console and to convert it to and from CSV format.
