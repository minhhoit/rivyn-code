#!/usr/bin/env bash
set -e

# ==============================================================================
# Rivyn Skill Sync Script for Aizen CLI
# Đồng bộ tất cả các skill từ repo rivyn-skill sang ~/.aizen/skills
# ==============================================================================

RIVYN_SKILL_DIR="${RIVYN_SKILL_PATH:-$HOME/Works/goal/rivyn-skill}"
AIZEN_SKILLS_DIR="$HOME/.aizen/skills"

echo "🔄 Bắt đầu đồng bộ skills từ: $RIVYN_SKILL_DIR"
echo "📂 Thư mục đích: $AIZEN_SKILLS_DIR"

if [ ! -d "$RIVYN_SKILL_DIR" ]; then
  echo "❌ Không tìm thấy thư mục: $RIVYN_SKILL_DIR"
  echo "   Vui lòng kiểm tra đường dẫn hoặc set RIVYN_SKILL_PATH=/duong/dan/rivyn-skill"
  exit 1
fi

mkdir -p "$AIZEN_SKILLS_DIR"

# 1. Tự động git pull cập nhật repo rivyn-skill
if [ -d "$RIVYN_SKILL_DIR/.git" ]; then
  echo "📥 Đang kéo cập nhật mới nhất (git pull)..."
  (cd "$RIVYN_SKILL_DIR" && git pull origin main 2>/dev/null || git pull 2>/dev/null || true)
fi

COUNT=0

# 2. Quét và đồng bộ các skill từ skills/ và examples/
for pattern in "$RIVYN_SKILL_DIR/skills"/* "$RIVYN_SKILL_DIR/examples"/*; do
  if [ -d "$pattern" ]; then
    name=$(basename "$pattern")
    # Kiểm tra SKILL.md
    if [ -f "$pattern/SKILL.md" ]; then
      cp "$pattern/SKILL.md" "$AIZEN_SKILLS_DIR/${name}.md"
      echo "  ✓ [Synced] $name"
      COUNT=$((COUNT + 1))
    elif [ -f "$pattern/skill.md" ]; then
      cp "$pattern/skill.md" "$AIZEN_SKILLS_DIR/${name}.md"
      echo "  ✓ [Synced] $name"
      COUNT=$((COUNT + 1))
    fi
  fi
done

# 3. Đồng bộ file SKILL.md gốc nếu có (Rivyn Cognitive OS Forge)
if [ -f "$RIVYN_SKILL_DIR/SKILL.md" ]; then
  cp "$RIVYN_SKILL_DIR/SKILL.md" "$AIZEN_SKILLS_DIR/rivyn-skill-forge.md"
  echo "  ✓ [Synced] rivyn-skill-forge"
  COUNT=$((COUNT + 1))
fi

echo ""
echo "✨ Đã đồng bộ thành công $COUNT skills vào Aizen CLI ($AIZEN_SKILLS_DIR)!"
echo "💡 Bạn có thể gõ \`/skills\` hoặc \`/works\` trong Aizen để sử dụng ngay."
