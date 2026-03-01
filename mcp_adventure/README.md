# 🎮 MCP Adventure Game

A **fun interactive demonstration** of MCP (Model Context Protocol) server filesystem operations!

## 🌟 What This Demonstrates

This adventure game uses MCP filesystem operations as game mechanics:

| MCP Operation | Game Mechanic |
|--------------|---------------|
| `create_directory` | Building game world rooms |
| `write_file` | Saving game state & creating items |
| `read_file` | Discovering secrets & exploring |
| `get_file_info` | Puzzle clues from metadata |
| `list_directory` | Inventory management |
| `search_files` | Finding hidden treasures |

## 🚀 Quick Start

```bash
cd mcp_adventure
python3 game.py
```

## 🎯 Game Features

### 🏰 Explore Three Rooms
- **Entrance**: Mysterious terminals and cryptic notes
- **Server Room**: Humming machines with network secrets
- **Vault**: Digital treasure protected by file metadata puzzles

### 🎒 Collect Items
- Ancient USB drives
- Network cables
- Admin badges
- Golden commits
- Master keys

### 🧩 Solve Puzzles
Use file metadata (size, timestamps, permissions) to unlock secrets!

## 🛠️ Technical Implementation

### Clean Code Principles Applied:

1. **Single Responsibility**: Each method does one thing
   - `initialize_game()` - Setup only
   - `explore_room()` - Room logic only
   - `collect_item()` - Inventory management only

2. **Type Hints**: Clear parameter types using `pathlib.Path`

3. **Error Handling**: Graceful file existence checks

4. **DRY Principle**: Reusable room exploration logic

5. **Clear Naming**: Methods describe exactly what they do

### MCP Operations Used:

```python
# Directory operations
Path.mkdir(exist_ok=True)  # → mcp__filesystem__create_directory

# File writing
with open(file, 'w') as f:  # → mcp__filesystem__write_file
    json.dump(data, f)

# File reading  
with open(file, 'r') as f:  # → mcp__filesystem__read_file
    data = json.load(f)

# Metadata extraction
stats = file.stat()  # → mcp__filesystem__get_file_info

# File searching
Path.rglob('*.json')  # → mcp__filesystem__search_files
```

## 🎨 Why This Is Fun

1. **Interactive**: Generates live game content
2. **Visual**: ASCII art treasure maps
3. **Educational**: Shows MCP in action
4. **Practical**: Real-world file operation patterns
5. **Extensible**: Easy to add more rooms/puzzles

## 🔧 Extend the Game

Add new rooms:
```python
rooms["library"] = {
    "description": "Dusty code archives",
    "items": ["ancient_documentation"],
    "secrets": "README files contain wisdom"
}
```

Add new mechanics:
- File permissions as lock combinations
- Directory depth as maze complexity
- File content as cipher keys

## 📊 Expected Output

```
🎮 Welcome to MCP ADVENTURE! 🎮
==================================================

🌟 Initializing MCP Adventure World...
✅ Game world created!

👤 Player: Code Explorer
📍 Starting location: entrance

🚪 Exploring the entrance...
📝 A mysterious room filled with glowing terminals
🎁 Items found: ancient_usb_drive, cryptic_note
🔮 Secret: The walls whisper: 'Check file timestamps for the code'
📊 File metadata: Size=XXX bytes

✅ Collected: ancient_usb_drive
✅ Collected: cryptic_note

🗺️  Generating treasure map...
📍 Map saved to: treasure_map.txt

📊 Game Statistics:
   total_rooms: 3
   inventory_items: 2
   save_exists: True

🎉 MCP Adventure Demo Complete! 🎉
```

## 💡 Key Takeaways

This demonstrates that **MCP filesystem operations** can:
- Build complex application state
- Create rich interactive experiences  
- Handle multiple file types (JSON, TXT)
- Extract and use metadata creatively
- Organize data hierarchically

**Clean code + MCP = Powerful possibilities!** 🚀
