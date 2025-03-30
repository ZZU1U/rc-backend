# API Documentation

## User endpoints

### GET /user
- Description: Получить всех юзеров
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
  {"id": "uuid-uuid-uuid"}
  ```

### POST /login
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
    "token": "sigma.balls.abacaba",
    "type": "Bearer"
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
      "name": "DaBaby Car",
      "description": "aoua",
      "image_url": "https://example.dina",
      "ip": "127.0.0.1"
    }
  ]
  ```

### POST /car
- Description: Создать машинку
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
