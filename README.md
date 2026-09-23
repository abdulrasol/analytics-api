# Google Analytics 4 (GA4) Data API Microservice 🦀

A lightweight, blazing-fast, and secure microservice written in **Rust** to pull reporting data dynamically from the **Google Analytics Data API (GA4)**. 

This project is built to be a **Universal Standard (Generic Proxy)**. Any developer can use it to fetch analytics for their own apps, websites, or dashboards without hardcoding any specific events inside the server.

## 🌟 Why this microservice?
Querying GA4 Data APIs from mobile or web clients directly requires exposing your Google Cloud Service Account, which is a massive security risk. 
This microservice acts as a highly optimized, ultra-lightweight proxy:
- Your client app sends a request containing the `property_id`, `event_name`, and `dimension` along with a static `API_KEY`.
- This service handles Google OAuth2 authentication securely, caches the Bearer token, fetches the requested GA4 data, and returns the pure JSON response.

---

## 🚀 Features
- **Ultra-lightweight:** Minimal Docker image size and RAM usage.
- **Secure:** Endpoint is protected by an `Authorization: Bearer <API_KEY>`.
- **CORS Enabled:** Fully supports Cross-Origin Resource Sharing (CORS) out of the box, allowing seamless integration with web browsers (e.g. Flutter Web, React).
- **Automatic Token Management:** Automatically generates and caches the OAuth2 token.
- **100% Dynamic & Generic:** It does not hardcode any app domains or events. You can query **any** built-in or custom event you have set up in GA4.


---

## 🛠️ Setup and Deployment

### 1. Prerequisites
- Docker installed.
- A Google Cloud Project with the **Google Analytics Data API** enabled.

### 2. Get your Service Account
1. Go to your Google Cloud Console.
2. Create a Service Account and generate a new private key (`.json`).
3. **Important:** Add this Service Account's email address as a "Viewer" in your Google Analytics Property Settings!
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

## 📊 GA4 Dashboard Guide (Crucial for Developers)

To use this API correctly, you must understand the difference between **Event Names** and **Custom Dimensions (Parameters)** in GA4:

1. **Event Name (`eventName`):** The action the user took (e.g., `screen_view`, `notification_opened`, `brand_click`). You **do not** need to register these in GA4. They are tracked automatically as the built-in `eventName` dimension.
2. **Event Parameters:** The specific details of that action (e.g., `item_name=LG`, `notification_title=Summer Sale`). 

**How to track specific parameters (Custom Dimensions):**
If you want to know *which* specific item or notification was clicked, you must register that parameter in your GA4 Dashboard:
1. Go to **Admin** (⚙️) > **Custom definitions**.
2. Click **Create custom dimension**.
3. **Dimension name:** Give it a readable name (e.g., "Item Name").
4. **Scope:** `Event`.
5. **Event parameter:** MUST exactly match the parameter your app sends (e.g., `item_name` or `notification_title`).

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
  "property_id": "YOUR_GA4_PROPERTY_ID",
  "event_name": "notification_opened",
  "dimension": "customEvent:notification_title"
}
```

### Dynamic Parameters Explanation:
- `property_id`: **(Required)** Your GA4 Property ID (found in GA4 Property Settings).
- `event_name`: *(Optional)* The exact name of the event to filter by.
- `dimension`: *(Optional)* The dimension to group your data by.
   - **Built-in dimensions:** `eventName`, `pageTitle` (for screens), `country`, etc.
   - **Custom dimensions:** If you registered a custom parameter (like `item_name`), you **must prefix it** with `customEvent:`, like this: `customEvent:item_name`.

#### 💡 Examples:

**1. Get Top Visited Screens:**
```json
{
  "property_id": "123456789",
  "event_name": "screen_view",
  "dimension": "pageTitle"
}
```
*(No custom dimension setup required in GA4)*

**2. Get Clicks per Brand (Custom Event):**
```json
{
  "property_id": "123456789",
  "event_name": "brand_click",
  "dimension": "customEvent:item_name"
}
```
*(Requires registering `item_name` as a Custom Dimension in GA4)*

---
## 💖 Contributing
Feel free to open issues and pull requests to improve this microservice.
