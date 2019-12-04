# Test task for trlogic

# Build & start
```bash
docker-compose up --build
``` 

Web server: http://127.0.0.1:8888

Url for uploading: http://127.0.0.1:8888/api/v1/image/upload

Json scheme:
```json
{
  "files": ["<data uri encoded in base64 by rfc2397>"],
  "urls": ["<url to image>"],
}
```
