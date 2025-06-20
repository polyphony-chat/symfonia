#!/bin/bash

# Check if correct number of arguments provided
if [ $# -ne 2 ]; then
    echo "Usage: $0 <start_number> <add_number>"
    echo "Example: $0 00005 2"
    exit 1
fi

start_num="$1"
add_num="$2"
directory="crates/util/migrations"

# Convert start_num to integer for comparison
start_int=$(echo "$start_num" | sed 's/^0*//')
start_int=${start_int:-0}  # Handle case where start_num is all zeros

# Check if directory exists
if [ ! -d "$directory" ]; then
    echo "Directory $directory does not exist"
    exit 1
fi

# Find files matching pattern and filter by number >= start_num
files_to_rename=()
while IFS= read -r -d '' file; do
    filename=$(basename "$file")
    if [[ $filename =~ ^([0-9]{5})_(.+)\.sql$ ]]; then
        file_num="${BASH_REMATCH[1]}"
        file_int=$(echo "$file_num" | sed 's/^0*//')
        file_int=${file_int:-0}
        
        if [ "$file_int" -ge "$start_int" ]; then
            files_to_rename+=("$filename:$file_int")
        fi
    fi
done < <(find "$directory" -maxdepth 1 -name "*.sql" -print0)

# Sort files by number (descending to avoid conflicts during renaming)
IFS=$'\n' sorted_files=($(printf '%s\n' "${files_to_rename[@]}" | sort -t: -k2 -nr))

# Rename files
for entry in "${sorted_files[@]}"; do
    filename="${entry%:*}"
    file_num="${entry#*:}"
    
    new_num=$((file_num + add_num))
    new_filename=$(printf "%05d_%s" "$new_num" "${filename:6}")
    
    old_path="$directory/$filename"
    new_path="$directory/$new_filename"
    
    echo "Renaming $old_path to $new_path"
    git mv "$old_path" "$new_path"
done

echo "Done!"
