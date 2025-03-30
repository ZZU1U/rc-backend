# API Documentation

## User endpoints

### GET /user/check
- Description: Проверить токен
- Response: CODE: 200
  
### GET /user
- Description: Получить всех юзеров
- Requirements: Must be super
- Response:
  ```json
  [
    {
      "id": "naufnasdiufnapfab23",
      "username": "John Doe",
      "create_time": "2025-01-12",
      "is_super": false
    }
  ]
  ```

### POST /user
- Description: Создать юзера
- Body:
  ```json
  {
    "username": "Joe Peach",
    "password": "john@example.com"
    "is_super": true // OPTIONAL
  }
  ```
- Response:
  ```json
  {
    "id": "uuid-uuid-uuid"
  }
  ```

### POST /user/jwt
- Description: Получить токен
- Body:
  ```json
  {
    "username": "Joe Peach",
    "password": "john@example.com"
  }
  ```
- Response:
  ```json
  {
    "access_token": "sigma.balls.abacaba",
    "token_type": "Bearer"
  }
  ```

## Car endpoints

### GET /car
- Description: Получить всех машинок
- Response:
  ```json
  [
    {
      "id": "naenf-1d1241s241-wsda",
      "create_time": "2025-01-12",
      "last_seen": "2025-01-12",
      "name": "DaBaby Car",
      "description": "aoua",
      "image_url": "https://example.dina"
    }
  ]
  ```

### POST /car
- Description: Создать машинку
- Requirements: Must be super
- Body:
  ```json
  {
    "name": "DaBaby Car",
    "description": "aoua", // OPTIONAL
    "image_url": "https://example.dina", // OPTIONAL
    "key": "cho-to"
  }
  ```
- Response:
  ```json
  {"id": "uuid-uuid-uuid"}
  ```
