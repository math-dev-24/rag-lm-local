# AIConsole

A Rust-based AI console application that provides an interactive chat interface with AI models through LM Studio, featuring vector storage capabilities with Qdrant.

## 🚀 Features

- **Interactive AI Chat**: Real-time conversation with AI models
- **LM Studio Integration**: Connect to local LM Studio instances
- **Vector Storage**: Qdrant integration for embedding storage
- **Model Management**: Dynamic model loading and switching
- **Health Monitoring**: Service health checks
- **Clean Architecture**: Hexagonal architecture with ports and adapters

## 🏗️ Architecture

The project follows a clean hexagonal architecture pattern:

```
src/
├── main.rs              # Application entry point
├── models/              # Data structures
│   └── ai.rs           # AI-related models
├── port/               # Ports (interfaces)
│   ├── ai_port.rs      # AI service interface
│   └── repository_port.rs
├── adapter/            # Adapters (implementations)
│   ├── ai/
│   │   ├── lm_studio.rs # LM Studio AI adapter
│   │   └── mod.rs
│   └── repository/
│       ├── qdrant.rs   # Qdrant vector database adapter
│       └── mod.rs
└── service/            # Business logic
    ├── ai_service.rs   # AI service implementation
    ├── qdrant_service.rs # Qdrant service
    └── mod.rs
```

## 📋 Prerequisites

- Rust (latest stable version)
- LM Studio running locally on port 1234
- Qdrant vector database instance
- Environment variables configured

## ⚙️ Setup

1. **Clone the repository**
   ```bash
   git clone <repository-url>
   cd AIConsole
   ```

2. **Install dependencies**
   ```bash
   cargo build
   ```

3. **Configure environment variables**
   Create a `.env` file in the project root:
   ```env
   QDRANT_URL=your_qdrant_url
   QDRANT_API_KEY=your_qdrant_api_key
   ```

4. **Start LM Studio**
   - Launch LM Studio
   - Start a local server on port 1234
   - Load your preferred AI model

5. **Start Qdrant**
   - Ensure Qdrant is running and accessible
   - Verify your API key has proper permissions

## 🚀 Usage

1. **Run the application**
   ```bash
   cargo run
   ```

2. **Interactive chat**
   - The application will connect to LM Studio and Qdrant
   - Available models will be loaded automatically
   - Start chatting with the AI model
   - Type `exit` to quit

## 🔧 Configuration

### AI Configuration
- **Base URL**: Defaults to `http://127.0.0.1:1234` for LM Studio
- **Model Selection**: Automatically selects the first available model
- **Chat History**: Maintains conversation context

### Qdrant Configuration
- **URL**: Set via `QDRANT_URL` environment variable
- **API Key**: Set via `QDRANT_API_KEY` environment variable
- **Collections**: Automatically retrieves available collections

## 📦 Dependencies

- **reqwest**: HTTP client for API communication
- **tokio**: Async runtime
- **serde**: Serialization/deserialization
- **qdrant-client**: Qdrant vector database client
- **async-trait**: Async trait support
- **dotenv**: Environment variable management

## 🏛️ Architecture Details

### Ports (Interfaces)
- `AiPort`: Defines AI service contract
- `RepositoryPort`: Defines data access contract

### Adapters (Implementations)
- `LmStudio`: LM Studio AI service adapter
- `QdrantRepository`: Qdrant vector database adapter

### Services (Business Logic)
- `AiService`: Orchestrates AI interactions
- `QdrantService`: Manages vector database operations

### Models (Data Structures)
- `AiChatMessage`: Chat message structure
- `AiChatRequest`: AI request format
- `AiConfig`: AI service configuration

## 🔍 Health Checks

The application performs health checks on startup:
- LM Studio connectivity
- Model availability
- Qdrant connection
- Collection retrieval

## 🛠️ Development

### Adding New AI Providers
1. Implement the `AiPort` trait
2. Add the adapter to the `adapter/ai/` directory
3. Update the main application to use the new adapter

### Adding New Vector Databases
1. Implement the `RepositoryPort` trait
2. Add the repository to the `adapter/repository/` directory
3. Create corresponding service in `service/`

## 📝 License

[Add your license information here]

## 🤝 Contributing

[Add contribution guidelines here]
