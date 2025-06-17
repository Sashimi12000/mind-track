#!/usr/bin/env python3
"""
仕様書ナンバリング統一スクリプト

このスクリプトは、specifications/ ディレクトリ内の全てのMarkdownファイルに対して、
統一的なナンバリングシステムを適用します。

ナンバリング体系:
- DC-X.Y.Z: Daily Checkin (01_daily_checkin.md)
- MT-X.Y.Z: Micro Task (02_micro_task_planner.md)
- AF-X.Y.Z: Achievement Feedback (03_achievement_log_feedback.md)
- RF-X.Y.Z: Reflection Feature (04_reflection_feature.md)
- RM-X.Y.Z: Reminder (05_reminder_feature.md)
- AR-X.Y.Z: Architecture (06_architecture_overview.md)
- DV-X.Y.Z: Development (07_development_considerations.md)
- FE-X.Y.Z: Future Extensions (08_future_extensions.md)
- GL-X.Y.Z: Glossary (09_glossary.md)
- IS-X.Y.Z: Implementation Strategy (10_implementation_strategy.md)
"""

import os
import re
from pathlib import Path
from typing import Dict, List, Tuple

# ファイル名とプレフィックスのマッピング
FILE_PREFIX_MAP = {
    "01_daily_checkin.md": "DC",
    "02_micro_task_planner.md": "MT", 
    "03_achievement_log_feedback.md": "AF",
    "04_reflection_feature.md": "RF",
    "05_reminder_feature.md": "RM",
    "06_architecture_overview.md": "AR",
    "07_development_considerations.md": "DV",
    "08_future_extensions.md": "FE",
    "09_glossary.md": "GL",
    "10_implementation_strategy.md": "IS"
}

def get_numbering_counters():
    """各レベルのナンバリングカウンターを初期化"""
    return {
        'level1': 1,  # ## レベル
        'level2': 1,  # ### レベル  
        'level3': 1,  # #### レベル
    }

def reset_lower_counters(counters: Dict[str, int], current_level: int):
    """現在のレベルより下位のカウンターをリセット"""
    if current_level <= 1:
        counters['level2'] = 1
        counters['level3'] = 1
    elif current_level <= 2:
        counters['level3'] = 1

def update_heading(line: str, prefix: str, counters: Dict[str, int]) -> str:
    """見出し行を新しいナンバリング形式に更新"""
    
    # 既に正しい形式の場合はスキップ
    if f"{prefix}-" in line and "{#" in line:
        return line
    
    # 見出しレベルを判定
    level = 0
    if line.startswith("### "):
        level = 3
    elif line.startswith("## "):
        level = 2
    elif line.startswith("# "):
        level = 1
    else:
        return line
    
    # カウンターを更新
    if level == 1:
        counters['level1'] = 1
        reset_lower_counters(counters, 1)
    elif level == 2:
        reset_lower_counters(counters, 2)
        current_num = counters['level2']
        counters['level2'] += 1
    elif level == 3:
        current_num = counters['level3']
        counters['level3'] += 1
    
    # 既存の見出しテキストを抽出
    heading_match = re.match(r'^(#{1,6})\s*\*\*(.+?)\*\*', line)
    if not heading_match:
        return line
        
    hash_marks = heading_match.group(1)
    heading_text = heading_match.group(2)
    
    # 古いナンバリングを除去
    # "1. テキスト", "1.1. テキスト", "DC-1.1. テキスト" などの形式を除去
    clean_text = re.sub(r'^[A-Z]{2}-\d+(\.\d+)*\.\s*', '', heading_text)
    clean_text = re.sub(r'^\d+(\.\d+)*\.\s*', '', clean_text)
    
    # 新しいナンバリングを生成
    if level == 1:
        new_number = f"{prefix}-1"
        anchor_id = f"{prefix}-1"
    elif level == 2:
        new_number = f"{prefix}-1.{current_num}"
        anchor_id = f"{prefix}-1.{current_num}"
    elif level == 3:
        level2_num = counters['level2'] - 1  # level2は既にインクリメント済み
        new_number = f"{prefix}-1.{level2_num}.{current_num}"
        anchor_id = f"{prefix}-1.{level2_num}.{current_num}"
    
    # 新しい見出し行を構築
    new_line = f"{hash_marks} **{new_number}. {clean_text}** {{#{anchor_id}}}\n"
    
    return new_line

def process_file(file_path: Path, prefix: str) -> bool:
    """指定されたファイルのナンバリングを更新"""
    try:
        with open(file_path, 'r', encoding='utf-8') as f:
            lines = f.readlines()
        
        counters = get_numbering_counters()
        updated_lines = []
        modified = False
        
        for line in lines:
            # 見出し行の処理
            if re.match(r'^#{1,3}\s*\*\*', line):
                new_line = update_heading(line, prefix, counters)
                if new_line != line:
                    modified = True
                updated_lines.append(new_line)
            else:
                updated_lines.append(line)
        
        # ファイルが変更された場合のみ書き込み
        if modified:
            with open(file_path, 'w', encoding='utf-8') as f:
                f.writelines(updated_lines)
            return True
            
    except Exception as e:
        print(f"エラー: {file_path} の処理中にエラーが発生しました: {e}")
        return False
    
    return False

def main():
    """メイン処理"""
    specs_dir = Path("specifications")
    
    if not specs_dir.exists():
        print("specifications ディレクトリが見つかりません")
        return
    
    print("仕様書ナンバリング統一スクリプトを開始します...")
    print()
    
    updated_files = 0
    total_files = 0
    
    # ファイル名順でソートして処理
    for filename in sorted(FILE_PREFIX_MAP.keys()):
        file_path = specs_dir / filename
        
        if not file_path.exists():
            print(f"⚠️  {filename} が見つかりません")
            continue
            
        total_files += 1
        prefix = FILE_PREFIX_MAP[filename]
        
        print(f"📝 {filename} を処理中... (プレフィックス: {prefix})")
        
        if process_file(file_path, prefix):
            print(f"✅ {filename} を更新しました")
            updated_files += 1
        else:
            print(f"📄 {filename} は既に正しい形式です")
    
    print()
    print(f"処理完了: {updated_files}/{total_files} ファイルが更新されました")
    
    if updated_files > 0:
        print()
        print("更新されたナンバリング体系:")
        for filename, prefix in FILE_PREFIX_MAP.items():
            print(f"  {filename} → {prefix}-X.Y.Z 形式")

if __name__ == "__main__":
    main()