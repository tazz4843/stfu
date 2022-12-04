# coding=utf-8
# Utility script to parse the profanity word list and output several Rust files containing the word list
# in different formats.
# Usage: python parse_word_list.py <word_list.txt> <output_dir>

import csv
import sys

total_lengths = set()

def load_csv(filename):
    """
    Open a CSV file and return a tuple of the header row and the data rows.
    """
    with open(filename, 'r') as f:
        reader = csv.reader(f)
        header = next(reader)
        data = [row for row in reader]
        return header, data

# csv header
# text,canonical_form_1,canonical_form_2,canonical_form_3,category_1,category_2,category_3,severity_rating,severity_description

def group_by_severity(data):
    """
    Group the data by severity.
    """
    groups = {}
    for row in data:
        severity = row[8]
        if severity not in groups:
            groups[severity] = []
        severity_group = groups[severity]
        severity_group.append(row[0])
        for word in row[1:4]:
            if word == "": continue
            word = word.lower()
            if word not in severity_group:
                severity_group.append(word)
    if "" in groups:
        del groups[""]
    return groups

def group_by_category(data):
    """
    Group the data by category.

    Note: a single word can have multiple categories.
    """
    groups = {}
    for row in data:
        for category in row[4:7]:
            if category not in groups:
                groups[category] = []
            category_group = groups[category]
            category_group.append(row[0])
            for word in row[1:4]:
                if word == "": continue
                word = word.lower()
                if word not in category_group:
                    category_group.append(word)
    if "" in groups:
        del groups[""]
    return groups

def group_by_word(data):
    """
    Group the data by the canonical form of the word.

    As with categories, a single word can have multiple canonical forms.
    """
    groups = {}
    for row in data:
        for word in row[1:4]:
            if word not in groups:
                groups[word] = []
            word_group = groups[word]
            word_group.append(row)
    if "" in groups:
        del groups[""]
    return groups

def rustify_string(s):
    """
    Convert a string to a valid Rust identifier.

    This converts the string to uppercase, and removes non-alphanumeric characters (except for spaces).
    Then trims any duplicated spaces, and finally replaces spaces with underscores.
    """
    return '_'.join([x for x in s.upper().replace("/", "").replace("-", "_").replace(' ', '_').split('_') if x != ''])

def write_rust_file(filename, kind, data):
    """
    Write a Rust file containing the data.

    Data is a list of tuples, where the first element is the variable name and the second
    is the actual word list.
    """
    with open(filename, 'w') as f:
        f.write('//! {} list generated by parse_word_list.py\n'.format(kind))
        for name, words in data:
            total_lengths.add(len(words))
            f.write(f"pub const {rustify_string(name)}: [&str; {len(words)}] = [\n")
            for word in words:
                f.write(f'    "{word}",\n')
            f.write("];\n")

def main():
    header, data = load_csv(sys.argv[1])
    output_dir = sys.argv[2]

    severity_group = group_by_severity(data)
    write_rust_file(f'{output_dir}/severity.rs', 'severity', [(severity, rows) for severity, rows in severity_group.items()])

    category_groups = group_by_category(data)
    write_rust_file(f'{output_dir}/category.rs', 'category', [(category, rows) for category, rows in category_groups.items()])

    word_groups = group_by_word(data)
    write_rust_file(f'{output_dir}/word.rs', 'word', [(word, [row[0] for row in rows]) for word, rows in word_groups.items()])

    print(f"Total lengths: {sorted(total_lengths)}")


if __name__ == '__main__':
    main()
