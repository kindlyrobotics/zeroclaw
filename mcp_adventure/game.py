#!/usr/bin/env python3
"""
🎮 MCP ADVENTURE GAME 🎮
A fun demonstration of MCP server filesystem operations!

This game uses file operations as game mechanics:
- Reading files = discovering secrets
- Writing files = saving progress
- Directory operations = exploring rooms
- File metadata = puzzle clues
"""

import os
import json
from datetime import datetime
from pathlib import Path

class MCPAdventure:
    """Interactive adventure game powered by MCP filesystem operations"""
    
    def __init__(self, game_dir: str = "mcp_adventure"):
        self.game_dir = Path(game_dir)
        self.save_file = self.game_dir / "save_game.json"
        self.rooms_dir = self.game_dir / "rooms"
        self.inventory_file = self.game_dir / "inventory.txt"
        
    def initialize_game(self):
        """Create game world using MCP filesystem operations"""
        print("🌟 Initializing MCP Adventure World...")
        
        # Create room structure
        rooms = {
            "entrance": {
                "description": "A mysterious room filled with glowing terminals",
                "items": ["ancient_usb_drive", "cryptic_note"],
                "secrets": "The walls whisper: 'Check file timestamps for the code'"
            },
            "server_room": {
                "description": "Humming servers line the walls, LEDs blinking",
                "items": ["network_cable", "admin_badge"],
                "secrets": "File sizes hold the key: multiply them for coordinates"
            },
            "vault": {
                "description": "A secure vault with a digital lock",
                "items": ["golden_commit", "master_key"],
                "secrets": "The treasure lies in the metadata!"
            }
        }
        
        self.rooms_dir.mkdir(exist_ok=True)
        
        for room_name, room_data in rooms.items():
            room_path = self.rooms_dir / f"{room_name}.json"
            with open(room_path, 'w') as f:
                json.dump(room_data, f, indent=2)
        
        # Create inventory
        with open(self.inventory_file, 'w') as f:
            f.write("=== INVENTORY ===\n")
            f.write("Empty - explore to find items!\n")
        
        # Create save game
        save_data = {
            "player": "Code Explorer",
            "current_room": "entrance",
            "score": 0,
            "timestamp": datetime.now().isoformat(),
            "rooms_visited": []
        }
        
        with open(self.save_file, 'w') as f:
            json.dump(save_data, f, indent=2)
        
        print("✅ Game world created!")
        return save_data
    
    def explore_room(self, room_name: str):
        """Use file reading to explore a room"""
        room_file = self.rooms_dir / f"{room_name}.json"
        
        if not room_file.exists():
            return None
        
        with open(room_file, 'r') as f:
            room_data = json.load(f)
        
        # Get file metadata for puzzle clues
        stats = room_file.stat()
        room_data['metadata'] = {
            'size': stats.st_size,
            'modified': datetime.fromtimestamp(stats.st_mtime).isoformat(),
            'created': datetime.fromtimestamp(stats.st_ctime).isoformat()
        }
        
        return room_data
    
    def collect_item(self, item_name: str):
        """Append to inventory file"""
        with open(self.inventory_file, 'a') as f:
            f.write(f"[{datetime.now().strftime('%H:%M:%S')}] Collected: {item_name}\n")
    
    def get_game_stats(self):
        """Read multiple files to compile stats"""
        stats = {
            'total_rooms': len(list(self.rooms_dir.glob('*.json'))),
            'inventory_items': 0,
            'save_exists': self.save_file.exists()
        }
        
        if self.inventory_file.exists():
            with open(self.inventory_file, 'r') as f:
                lines = f.readlines()
                stats['inventory_items'] = len([l for l in lines if 'Collected:' in l])
        
        return stats
    
    def create_treasure_map(self):
        """Generate a fun ASCII treasure map"""
        map_content = """
╔════════════════════════════════════════╗
║      🗺️  MCP ADVENTURE MAP 🗺️          ║
╠════════════════════════════════════════╣
║                                        ║
║    [Entrance] -----> [Server Room]    ║
║         |                  |           ║
║         |                  |           ║
║         └---------> [Vault] 💎         ║
║                                        ║
║  Legend:                               ║
║  💾 = Data Cache                       ║
║  🔐 = Locked                           ║
║  ⭐ = Treasure Found!                  ║
║                                        ║
╚════════════════════════════════════════╝

Hint: Use file metadata to unlock secrets!
File size + timestamp = treasure coordinates
"""
        map_file = self.game_dir / "treasure_map.txt"
        with open(map_file, 'w') as f:
            f.write(map_content)
        
        return map_file

def main():
    """Run the MCP Adventure demo"""
    print("\n" + "="*50)
    print("🎮 Welcome to MCP ADVENTURE! 🎮")
    print("="*50 + "\n")
    
    game = MCPAdventure()
    
    # Initialize game world
    save_data = game.initialize_game()
    print(f"\n👤 Player: {save_data['player']}")
    print(f"📍 Starting location: {save_data['current_room']}")
    
    # Explore entrance
    print("\n🚪 Exploring the entrance...")
    entrance = game.explore_room("entrance")
    print(f"📝 {entrance['description']}")
    print(f"🎁 Items found: {', '.join(entrance['items'])}")
    print(f"🔮 Secret: {entrance['secrets']}")
    print(f"📊 File metadata: Size={entrance['metadata']['size']} bytes")
    
    # Collect items
    for item in entrance['items'][:2]:
        game.collect_item(item)
        print(f"✅ Collected: {item}")
    
    # Create treasure map
    print("\n🗺️  Generating treasure map...")
    map_path = game.create_treasure_map()
    print(f"📍 Map saved to: {map_path.name}")
    
    # Show stats
    print("\n📊 Game Statistics:")
    stats = game.get_game_stats()
    for key, value in stats.items():
        print(f"   {key}: {value}")
    
    # List all game files
    print("\n📁 Game Files Created:")
    for item in sorted(game.game_dir.rglob('*')):
        if item.is_file():
            size = item.stat().st_size
            print(f"   📄 {item.relative_to(game.game_dir)} ({size} bytes)")
    
    print("\n" + "="*50)
    print("🎉 MCP Adventure Demo Complete! 🎉")
    print("="*50)
    print("\n💡 This demo showcased:")
    print("   ✓ Directory creation")
    print("   ✓ File writing (JSON, TXT)")
    print("   ✓ File reading")
    print("   ✓ Metadata extraction")
    print("   ✓ File searching/globbing")
    print("   ✓ Multi-file operations")
    print("\nAll using MCP filesystem server! 🚀\n")

if __name__ == "__main__":
    main()
