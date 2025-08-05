# dumq_amqp Documentation

이 디렉토리는 dumq_amqp 라이브러리에 대한 문서를 포함합니다.

## 문서 목록

### [API.md](API.md)
완전한 API 참조 문서입니다. 모든 모듈, 타입, 함수에 대한 상세한 설명과 예제를 포함합니다.

**주요 내용:**
- Core Types (AmqpValue, AmqpSymbol, AmqpList, AmqpMap)
- Connection Management (Connection, ConnectionBuilder, ConnectionState)
- Session Management (Session, SessionState)
- Link Management (Sender, Receiver, LinkBuilder)
- Message System (Message, Header, Properties, Body)
- Encoding/Decoding (Encoder, Decoder)
- Error Handling (AmqpError, AmqpResult)
- Transport Layer (Transport, Frame)
- Best Practices
- Complete Examples

### [USER_GUIDE.md](USER_GUIDE.md)
사용자 가이드입니다. MAMQP를 사용하여 메시징 애플리케이션을 구축하는 방법을 단계별로 설명합니다.

**주요 내용:**
- Getting Started
- Basic Concepts (AMQP 1.0 아키텍처)
- Connection Management
- Sending Messages
- Receiving Messages
- Error Handling
- Advanced Features
- Troubleshooting
- Best Practices
- Performance Tips
- Complete Examples

## 빠른 시작

### 설치

`Cargo.toml`에 MAMQP를 추가하세요:

```toml
[dependencies]
dumq_amqp = "0.1.0"
tokio = { version = "1.0", features = ["full"] }
```

### 기본 예제

```rust
use dumq_amqp::prelude::*;
use tokio::time::Duration;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 연결 생성
    let mut connection = ConnectionBuilder::new()
        .hostname("localhost")
        .port(5672)
        .timeout(Duration::from_secs(30))
        .container_id("my-app")
        .build();

    // 연결 열기
    connection.open().await?;

    // 세션 생성
    let mut session = connection.create_session().await?;
    session.begin().await?;

    // 송신자 생성
    let mut sender = LinkBuilder::new()
        .name("my-sender")
        .target("my-queue")
        .build_sender(session.id().to_string());

    sender.attach().await?;
    sender.add_credit(10);

    // 메시지 전송
    let message = Message::text("Hello, AMQP!");
    let delivery_id = sender.send(message).await?;
    println!("Message sent with delivery ID: {}", delivery_id);

    // 정리
    sender.detach().await?;
    session.end().await?;
    connection.close().await?;

    Ok(())
}
```

## 문서 생성

로컬에서 문서를 생성하려면:

```bash
# 라이브러리 문서 생성
cargo doc --no-deps

# 문서를 브라우저에서 열기
cargo doc --no-deps --open
```

## 테스트

```bash
# 단위 테스트 실행
cargo test

# 예제 실행
cargo run
```

## 기능

- **완전한 AMQP 1.0 프로토콜 지원**: AMQP 1.0 명세의 완전한 구현
- **Async/Await**: Tokio 기반의 고성능 비동기 작업
- **타입 안전성**: 강력한 타입의 AMQP 값과 메시지
- **Builder 패턴**: 쉬운 설정을 위한 Fluent Builder API
- **오류 처리**: 상세한 오류 메시지와 포괄적인 오류 타입
- **확장 가능**: 쉬운 확장과 커스터마이징을 위한 모듈형 설계

## 라이센스

이 프로젝트는 MIT 라이센스 하에 배포됩니다. 자세한 내용은 [LICENSE](../LICENSE) 파일을 참조하세요.

## 기여

기여를 환영합니다! Pull Request를 자유롭게 제출해 주세요. 큰 변경사항의 경우 먼저 이슈를 열어서 논의해 주세요. 