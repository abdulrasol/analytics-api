# Google Analytics 4 (GA4) Microservice 🦀

A lightweight, blazing-fast, and secure microservice written in **Rust** to pull reporting data from **Google Analytics Data API (GA4)**.

## 🌟 Why this microservice?
Querying GA4 Data APIs from mobile or web clients directly requires exposing your Google Cloud Service Account, which is a massive security risk. 
This microservice acts as a highly optimized, ultra-lightweight proxy:
- Your app sends a request containing the `property_id` and the `report_type` along with a static `API_KEY`.
- This service handles Google OAuth2 authentication securely, caches the Bearer token, fetches the requested GA4 data, and returns the pure JSON response.

## 🚀 Features
- **Ultra-lightweight:** Minimal Docker image size and RAM usage.
- **Secure:** Endpoint is protected by an `Authorization: Bearer <API_KEY>`.
- **Automatic Token Management:** Automatically generates and caches the OAuth2 token for Google APIs.
- **Dynamic Projects:** Accepts `property_id` dynamically inside the request payload, making it generic for multiple apps/websites.

---

## 🛠️ Setup and Deployment

### 1. Prerequisites
- [Docker](https://docs.docker.com/get-docker/) installed.
- A Google Cloud Project with the **Google Analytics Data API** enabled.

### 2. Get your Service Account
1. Go to your Google Cloud Console.
2. Create a Service Account and generate a new private key (`.json`).
3. **Important:** Add this Service Account's email address as a "Viewer" in your Google Analytics Property settings!
4. Rename the downloaded file to `service-account.json` and place it in the root directory.

### 3. Configure the Environment
```bash
cp .env.example .env
```
Edit `.env` and fill in your details:
```env
PORT=8080
API_KEY=my_super_secret_api_key_123
GOOGLE_APPLICATION_CREDENTIALS=/app/service-account.json
```

### 4. Run the Service (Docker)
```bash
docker-compose up -d --build
```
*Your service is now running!*

---

## 📡 API Usage

### Endpoint: `POST /api/v1/analytics/report`

**Headers:**
```http
Content-Type: application/json
Authorization: Bearer my_super_secret_api_key_123
```

**Body (JSON):**
```json
{
  "property_id": "553872953",
  "report_type": "overview"
}
```

#### Supported `report_type`:
- `"overview"`: General daily active users and events count.
- `"brands"`: Clicks and events related to brands.
- `"screens"`: Screen views.

#### Example using `cURL`:
```bash
curl -X POST http://localhost:8083/api/v1/analytics/report \
  -H "Content-Type: application/json" \
  -H "Authorization: Bearer my_super_secret_api_key_123" \
  -d '{
    "property_id": "123456789",
    "report_type": "overview"
  }'
```
