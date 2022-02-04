pub fn print_help() {
    println!("
    USAGE:

        przt [--decode] [arguments]

    Przyczajony-tygrys hides a file in alpha channel of an image.
    The image has to have the alpha channel added and be in a PNG format for now.
    Additionally the image must be big enough to fit the file.
    The capacity is => width x height = file size in bytes + 4.

    ARGUMENTS:

        --decode    - Enables decoding mode.
        -i <path>   - Path to the input image, required in encoding and decoding.
        -f <path>   - Path to the file that is to be hidden, only required in encoding mode.
        -o <path>   - Path to where the result is going to be saved.
                      For encoding its the image and for decoding the hidden file.
        -h          - Get this help information.
    ");
}
