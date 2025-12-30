"""
audit_rust_sdd_gap.py

A utility script to perform a gap analysis between the Rust codebase
and the Security Design Description (SDD) document.

Usage:
    python audit_rust_sdd_gap.py --code ./crates --sdd ./docs/design/SDD.md --output gap_report_rust.json
"""

import os
import re
import json
import argparse
from typing import Set, Dict, List
from pathlib import Path

class RustCodeAnalyser:
    """
    Parses Rust source files to extract pub struct, enum, trait, and fn definitions.
    """
    def __init__(self):
        self.definitions: Set[str] = set()

    def parse_content(self, content: str):
        # Extract public structs, enums, traits
        # Example: pub struct OptionConfig { ... }
        types = re.findall(r'pub\s+(?:struct|enum|trait)\s+([a-zA-Z0-9_]+)', content)
        self.definitions.update(types)

        # Extract public functions (excluding impl blocks for simplicity in regex)
        # Example: pub fn calculate_cva(...)
        funcs = re.findall(r'pub\s+fn\s+([a-zA-Z0-9_]+)', content)
        self.definitions.update(funcs)

    def analyse_directory(self, root_path: str) -> Set[str]:
        for root, _, files in os.walk(root_path):
            for file in files:
                if file.endswith(".rs"):
                    full_path = os.path.join(root, file)
                    try:
                        with open(full_path, "r", encoding="utf-8") as f:
                            self.parse_content(f.read())
                    except Exception as e:
                        print(f"Warning: Could not read {full_path}: {e}")
        return self.definitions

class SDDParser:
    """
    Parses the Markdown SDD to extract mentioned modules and components.
    """
    def __init__(self, sdd_path: str):
        self.sdd_path = sdd_path
        self.mentioned_entities: Set[str] = set()

    def parse(self) -> Set[str]:
        path_obj = Path(self.sdd_path)
        if not path_obj.exists():
            # If SDD doesn't exist yet, return empty set to show all code as 'undocumented'
            print(f"Warning: SDD not found at {self.sdd_path}. All code will be reported as undocumented.")
            return set()

        with open(path_obj, "r", encoding="utf-8") as f:
            content = f.read()

        # Extract code blocks or backticked items (e.g., `CVAEngine`)
        code_snippets = re.findall(r'`([a-zA-Z0-9_]+)`', content)
        self.mentioned_entities.update(code_snippets)
        return self.mentioned_entities

class GapReporter:
    def __init__(self, code_entities: Set[str], sdd_entities: Set[str]):
        self.code_entities = code_entities
        self.sdd_entities = sdd_entities

    def generate_report(self) -> Dict[str, List[str]]:
        undocumented = sorted(list(self.code_entities - self.sdd_entities))
        unimplemented = sorted(list(self.sdd_entities - self.code_entities))

        return {
            "summary": {
                "total_code_entities": len(self.code_entities),
                "total_sdd_entities": len(self.sdd_entities),
                "undocumented_count": len(undocumented),
                "unimplemented_count": len(unimplemented)
            },
            "undocumented_items_in_code": undocumented,
            "unimplemented_items_in_sdd": unimplemented
        }

def main():
    parser = argparse.ArgumentParser(description="Audit Rust SDD Gap")
    parser.add_argument("--code", required=True, help="Path to crates directory")
    parser.add_argument("--sdd", required=True, help="Path to SDD markdown")
    parser.add_argument("--output", default="gap_report_rust.json", help="Output JSON path")
    args = parser.parse_args()

    analyser = RustCodeAnalyser()
    code_entities = analyser.analyse_directory(args.code)

    sdd_parser = SDDParser(args.sdd)
    sdd_entities = sdd_parser.parse()

    reporter = GapReporter(code_entities, sdd_entities)
    report = reporter.generate_report()

    with open(args.output, "w", encoding="utf-8") as f:
        json.dump(report, f, indent=4)

    print(f"Report saved to {args.output}")
    print(f"Undocumented Rust entities: {report['summary']['undocumented_count']}")

if __name__ == "__main__":
    main()