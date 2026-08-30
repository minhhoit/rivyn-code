# Hướng Dẫn Sử Dụng `/works`, Audit Hệ Thống & Cẩm Nang Chuyển Đổi Provider Trong Aizen CLI

Tài liệu này bao gồm 3 phần chính:
1. **Hướng dẫn Command `/works` & Tích hợp Skill từ `rivyn-skill`**.
2. **Báo cáo Audit Hệ thống Toàn diện & Lộ trình Cải tiến (System Audit & Improvement Roadmap)**.
3. **Cẩm nang Chuyển đổi & Tích hợp Các Nhà cung cấp Mô hình (Provider Switching & Multi-Model Integration)**.

---

## PHẦN 1: COMMAND `/works` VÀ CÁC THINKING FRAMEWORKS TỪ `rivyn-skill`

### 1. Giới thiệu Command `/works`
Command `/works` (các alias: `/biz`, `/business`, `/workspaces`) là trung tâm điều phối (Workspace & Strategy Hub) kết hợp giữa:
- **Executive Advisory Board (Hội đồng Cố vấn Chiến lược Đỉnh cao)**: Mô phỏng tư duy của Alex Hormozi, Steve Jobs, Elon Musk, Charlie Munger, Naval Ravikant.
- **Rivyn Skill Forge**: Giao thức chưng cất Cognitive Operating System từ bất kỳ chuyên gia nào.
- **Growth, Marketing & Product Execution**: Các playbook thực chiến về Product Launch (GTM), High-Converting Copywriting (PAS/AIDA), B2B SaaS Sales (MEDDPICC), và UI/UX Master Aesthetic (50+ styles).

### 2. Bảng Tra cứu Nhanh Lệnh `/works`

| Lệnh | Mô tả & Playbook kích hoạt |
|---|---|
| `/works` | Mở Dashboard điều phối trung tâm hiển thị toàn bộ cố vấn và playbook |
| `/works board` | Triệu tập Hội đồng Cố vấn Chiến lược (Hormozi, Jobs, Musk, Munger, Naval) |
| `/works hormozi` | Kích hoạt Alex Hormozi: Value Equation, Grand Slam Offers, $100M Leads, Rule of 100 |
| `/works jobs` | Kích hoạt Steve Jobs: Radical Focus, Nói KHÔNG với 100 ý tưởng, End-to-End Taste |
| `/works musk` | Kích hoạt Elon Musk: First Principles, Thuật toán Kỹ thuật 5 bước, Giới hạn Vật lý |
| `/works munger` | Kích hoạt Charlie Munger: Tư duy Đảo ngược (Inversion), 25 Thiên kiến Tâm lý |
| `/works naval` | Kích hoạt Naval Ravikant: Đòn bẩy Không Cần Xin Phép (Code/Media), Kiến thức Đặc thù |
| `/works forge` | Kích hoạt Rivyn Skill Forge: Chưng cất Cognitive OS từ một người bất kỳ thành Skill |
| `/works design` | Kích hoạt UI/UX Master Playbook: 50+ Modern Styles, 97 Bảng màu, 57 Cặp Font, Tailwind |
| `/works gtm` | Kích hoạt Go-To-Market & Product Launch Playbook: Product Hunt, Hacker News, Viral loop |
| `/works copy` | Kích hoạt Copywriting Chuyển đổi Cao: Công thức PAS / AIDA, Cấu trúc Hero Page |
| `/works sales` | Kích hoạt B2B SaaS Sales: Khung đánh giá MEDDPICC, Discovery Call, Định giá Tiers |
| `/goal <mục_tiêu>` | Chạy vòng lặp tự trị để hoàn thành mục tiêu kinh doanh/marketing đến khi xong |

---

## PHẦN 2: BÁO CÁO AUDIT TOÀN DIỆN & LỘ TRÌNH CẢI TIẾN

### 1. Đánh giá Kiến trúc Hiện tại

#### Điểm mạnh cốt lõi:
1. **Single Static Binary (Pure Rust, Rustls)**: Khởi động siêu tốc (<=10.8ms), không phụ thuộc runtime bên ngoài (không cần Node, Python, C dynamic libs).
2. **Context & Token Management**: Cơ chế tự động nén token (`/compact`), streaming token HUD, context window analyzer (`/context`), chi phí theo thời gian thực (`/cost`).
3. **Time Machine & Snapshots**: Lưu trữ checkpoint làm việc tự động mỗi turn, rollback code và hội thoại liền mạch (`/timemachine`, `/undo`, `/redo`).
4. **Hệ thống Ghi nhớ Thông minh (Semantic Memory Brain)**: 5 trục nhận thức (Identity, Preferences, Constraints, Codebase Rules, Decisions) với cơ chế Reconcile chống trùng lặp.
5. **Đa dạng Builtin Skills**: 22 builtin skills được nhúng trực tiếp, truy xuất O(1) không tốn thời gian đọc ổ đĩa.

#### Các điểm cần cải tiến (Areas for Improvement):
1. **Autonomous Multi-Agent Collaboration**:
   - *Hiện trạng:* Các agent chuyên gia hoạt động chủ yếu độc lập hoặc qua sub-agent forks.
   - *Cải tiến:* Hỗ trợ mô hình **"Debate & Consensus"** — cho phép 2 hay nhiều cố vấn (ví dụ Hormozi tranh biện với Jobs về pricing vs UX) trao đổi trực tiếp trong cùng 1 turn để đưa ra phương án tối ưu nhất.
2. **Dynamic Skill Forge Output Auto-Save**:
   - *Hiện trạng:* Chưng cất skill xuất ra Markdown, người dùng copy hoặc gõ lệnh lưu.
   - *Cải tiến:* Bổ sung tool `save_distilled_skill` cho phép agent sau khi chưng cất tự động ghi ngay vào `.aizen/skills/<name>.md` và nạp nóng (hot-reload) vào registry.
3. **External Model Proxy Auto-Detection**:
   - *Hiện trạng:* Người dùng cấu hình profile thủ công qua `/provider add` hoặc `aizen config provider add`.
   - *Cải tiến:* Tự động quét các biến môi trường phổ biến (`OPENAI_API_KEY`, `ANTHROPIC_API_KEY`, `OPENROUTER_API_KEY`, `DEEPSEEK_API_KEY`, `GEMINI_API_KEY`) khi khởi động lần đầu để tạo sẵn các profile tương ứng.

---

## PHẦN 3: CẨM NANG CHUYỂN ĐỔI & TÍCH HỢP PROVIDER MODEL (AI CODE)

Aizen được thiết kế theo chuẩn **OpenAI-compatible Chat Completions API + SSE Streaming**, cho phép kết nối trực tiếp với hầu hết mọi AI provider trên thế giới, bao gồm cả Claude, OpenAI, Antigravity, OpenRouter, DeepSeek, Groq, Ollama và vLLM.

### 1. Cách Chuyển đổi Nhanh Trong REPL (Giao diện Chat)

```text
/provider                -> Mở menu tương tác chọn trong danh sách provider đã cấu hình
/provider <tên>          -> Chuyển ngay sang provider đó (ví dụ: /provider openrouter)
/provider add            -> Wizard tương tác tạo một provider mới
/provider manage         -> Chỉnh sửa, đổi tên, xóa provider
/model                   -> Xem danh sách models của provider hiện tại và chọn model
/effort [auto|high|max]  -> Điều chỉnh mức độ suy luận reasoning (o3-mini, DeepSeek R1, Claude Extended Thinking)
```

### 2. Cách Cấu hình qua CLI Command (Terminal bên ngoài)

```bash
# 1. Liệt kê tất cả provider đã lưu (* là provider đang hoạt động)
aizen config provider list

# 2. Chuyển provider hoạt động
aizen config provider use openrouter
aizen config provider use openai
aizen config provider use deepseek

# 3. Thêm một provider mới
aizen config provider add openrouter \
  --url "https://openrouter.ai/api/v1" \
  --key "sk-or-v1-xxxxxxxxxxxxxxxxxxxx" \
  --model "anthropic/claude-3.7-sonnet"

# 4. Xem cấu hình hiện tại
aizen config show
```

---

### 3. Hướng Dẫn Tích Hợp Từng Nhà Cung Cấp Cụ Thể

#### A. OpenRouter (Khuyên dùng — Truy cập TẤT CẢ model: Claude 3.7, DeepSeek R1, GPT-4o, Gemini 2.0)
- **Base URL:** `https://openrouter.ai/api/v1`
- **Lấy API Key:** [openrouter.ai/keys](https://openrouter.ai/keys)
- **Thiết lập:**
  ```bash
  aizen config provider add openrouter \
    --url "https://openrouter.ai/api/v1" \
    --key "sk-or-v1-..." \
    --model "anthropic/claude-3.7-sonnet"
  aizen config provider use openrouter
  ```
- **Các model đề xuất:**
  - `anthropic/claude-3.7-sonnet` (Coding & suy luận tốt nhất hiện nay)
  - `anthropic/claude-3.7-sonnet:thinking` (Bật Extended Thinking)
  - `deepseek/deepseek-r1` (Suy luận chuyên sâu chi phí siêu rẻ)
  - `openai/gpt-4o` (Tốc độ cao và ổn định)
  - `google/gemini-2.5-pro-preview-03-25` (Context window 1M-2M tokens)

---

#### B. OpenAI Trực Tiếp
- **Base URL:** `https://api.openai.com/v1`
- **Lấy API Key:** [platform.openai.com/api-keys](https://platform.openai.com/api-keys)
- **Thiết lập:**
  ```bash
  aizen config provider add openai \
    --url "https://api.openai.com/v1" \
    --key "sk-proj-..." \
    --model "gpt-4o"
  aizen config provider use openai
  ```
- **Các model đề xuất:** `gpt-4o`, `o3-mini`, `o1`.

---

#### C. Claude / Anthropic (Claude Code Compatibility)
Anthropic sử dụng endpoint `/v1/messages`. Khi dùng Aizen với Anthropic, bạn có thể kết nối thông qua OpenRouter, Cloudflare AI Gateway, hoặc proxy cục bộ (như LiteLLM):
- **Qua OpenRouter (Đơn giản nhất):**
  - Model: `anthropic/claude-3.7-sonnet`
- **Qua LiteLLM Proxy (Chạy cục bộ):**
  ```bash
  # Cài và chạy LiteLLM
  pip install litellm
  export ANTHROPIC_API_KEY="sk-ant-..."
  litellm --model claude-3-7-sonnet-20250219 --port 4000
  ```
  ```bash
  # Cấu hình Aizen trỏ vào LiteLLM
  aizen config provider add claude-local \
    --url "http://localhost:4000/v1" \
    --key "sk-anything" \
    --model "claude-3-7-sonnet-20250219"
  aizen config provider use claude-local
  ```

---

#### D. Google Antigravity & Google Gemini
- **Qua OpenRouter:** Model `google/gemini-2.5-pro` hoặc `google/gemini-2.5-flash`.
- **Qua Google AI Studio (OpenAI-compatible endpoint):**
  - **Base URL:** `https://generativelanguage.googleapis.com/v1beta/openai/`
  - **Key:** Gemini API key từ Google AI Studio
  - **Model:** `gemini-2.0-flash` hoặc `gemini-2.5-pro-preview`
  ```bash
  aizen config provider add gemini \
    --url "https://generativelanguage.googleapis.com/v1beta/openai/" \
    --key "AIzaSy..." \
    --model "gemini-2.0-flash"
  aizen config provider use gemini
  ```

---

#### E. DeepSeek Trực Tiếp (Chi Phí Siêu Tiết Kiệm)
- **Base URL:** `https://api.deepseek.com/v1`
- **Thiết lập:**
  ```bash
  aizen config provider add deepseek \
    --url "https://api.deepseek.com/v1" \
    --key "sk-..." \
    --model "deepseek-chat"
  aizen config provider use deepseek
  ```
- **Các model:** `deepseek-chat` (V3), `deepseek-reasoner` (R1).

---

#### F. Groq (Tốc Độ Suy Luận Nhanh Nhất)
- **Base URL:** `https://api.groq.com/openai/v1`
- **Model:** `llama-3.3-70b-versatile`, `deepseek-r1-distill-llama-70b`
  ```bash
  aizen config provider add groq \
    --url "https://api.groq.com/openai/v1" \
    --key "gsk_..." \
    --model "llama-3.3-70b-versatile"
  aizen config provider use groq
  ```

---

#### G. Local Models / Tự Host Offline (Ollama, LM Studio, vLLM)
- **Ollama:**
  - Base URL: `http://localhost:11434/v1`
  - Key: `ollama`
  - Model: `qwen2.5-coder:32b`, `deepseek-r1:14b`
  ```bash
  aizen config provider add ollama \
    --url "http://localhost:11434/v1" \
    --key "ollama" \
    --model "qwen2.5-coder:32b"
  aizen config provider use ollama
  ```
- **LM Studio:**
  - Base URL: `http://localhost:1234/v1`
  - Key: `lm-studio`
  - Model: tên model đang load trong LM Studio

---

### 4. Thiết Lập Provider Qua Biến Môi Trường (Environment Variables)

Bạn có thể override nhanh endpoint cho 1 phiên làm việc:
```bash
export AIZEN_BASE_URL="https://openrouter.ai/api/v1"
export AIZEN_API_KEY="sk-or-v1-..."
export AIZEN_MODEL="anthropic/claude-3.7-sonnet"
export AIZEN_EFFORT="high"

aizen
```

### 5. Multi-Role Model Routing (Định tuyến Model theo Vai trò)

Aizen hỗ trợ chỉ định model riêng biệt cho từng vai trò tác vụ để tối ưu chi phí và tốc độ:
- **`AIZEN_SUMMARIZER_MODEL`**: Model tóm tắt ngữ cảnh khi context window đầy (khuyên dùng model nhẹ, rẻ như `openai/gpt-4o-mini` hoặc `google/gemini-2.0-flash`).
- **`AIZEN_ORACLE_MODEL`**: Model tham vấn giải quyết bài toán hóc búa (khuyên dùng `anthropic/claude-3.7-sonnet` hoặc `deepseek/deepseek-r1`).
- **`AIZEN_SUBAGENT_DEFAULT_MODEL`**: Model mặc định cho các sub-agents con chạy song song.
- **Per-Agent Provider Routing:**
  ```text
  /agents set-provider <tên-agent> <tên-provider> [model]
  ```
