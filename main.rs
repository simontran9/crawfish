//! Compiler CLI
use std::{env, fmt::Result};

/// Errors that occur when using the compiler as a command line tool
enum CLIError {
    InvalidCommand,
    InvalidFileExtension,
    IncorrectUsage,
    FileOpenFailure,
    FilReadFailure
}

/// Entrypoint for the compiler CLI
fn main() -> Result<(), CLIError> {
    let args: Vec<String> = env::args().collect();
    match args.len() {
        1 => Ok(help())
        _ => Err(CLIError::InvalidCommand)
    }
}

/// prints the help menu to STDOUT
fn help() {
    let message =
        r"Crawfish compiler
        Usage: crawfish [COMMAND] [ARGUMENT]
        Commands
            build [file].crw        Compile the current package\n
            run [file].crw          Run the file without compiling\n
            help                    Display possible commands\n
            version                 Display compiler version";
    println!("{}", message);
}

// prints the version to STDOUT
fn version() {
    let message = "crawfish 1.0.0";
    println!("{}", message);
}

//int main(int argc, char *argv[]) {
//    if (argc == 1) {
//        help();
//        return SUCCESS;
//    } else if (argc == 2) {
//        if (strcmp(argv[1], "version") == 0) {
//            version();
//            return SUCCESS;
//        } else if (strcmp(argv[1], "help") == 0) {
//            help();
//            return SUCCESS;
//        } else {
//            fputs("CLI Error: Invalid Command\n", stderr);
//            return INVALID_COMMAND;
//        }
//    } else if (argc == 3) {
//        const char *command = argv[1];
//        const char *file_path = argv[2];
//
//        if (strcmp(command, "build") != 0) {
//            fprintf(stderr, "CLI Error: Invalid Command\n");
//            return INVALID_COMMAND;
//        }
//
//        uint64_t file_path_length = strlen(file_path);
//        if (file_path_length < 4 || strcmp(file_path + file_path_length - 4, ".crw") != 0) {
//            fprintf(stderr, "CLI Error: Invalid File Extension\n");
//            return INVALID_FILE_EXTENSION;
//        }
//
//        CLIError result = compile(file_path);
//        if (result != SUCCESS) {
//            fprintf(stderr, "CLI Error: Compilation Failed\n");
//            return result;
//        }
//
//        return SUCCESS;
//    } else {
//        fprintf(stderr, "CLI Error: Incorrect Usage\n");
//        return INCORRECT_USAGE;
//    }
//}
//
//// attempts to compile the source code
//CLIError compile(const char *file_path) {
//    FILE *file = fopen(file_path, "r");
//    if (file == NULL) {
//        perror("Error opening file");
//        return FILE_OPEN_FAILURE;
//    }
//
//    fseek(file, 0, SEEK_END);
//    int64_t file_size = ftell(file);
//    if (file_size == -1L) {
//        perror("Error determining file size");
//        fclose(file);
//        return FILE_READ_FAILURE;
//    }
//    rewind(file);
//
//    char *file_content = (char *) calloc(file_size + 1, sizeof(char));
//    if (file_content == NULL) {
//        perror("Calloc system call error");
//        fclose(file);
//        return MEMORY_ALLOCATION_FAILURE;
//    }
//
//    uint64_t read_size = fread(file_content, 1, file_size, file);
//    if (read_size != (uint64_t) file_size) {
//        perror("Error reading file");
//        free(file_content);
//        fclose(file);
//        return FILE_READ_FAILURE;
//    }
//    file_content[file_size] = '\0';
//
//    // TODO
//    printf("Compiling file: %s...\n", file_path);
//    printf("File content:\n%s\n", file_content);
//
//    free(file_content);
//    fclose(file);
//
//    return SUCCESS;
//}
