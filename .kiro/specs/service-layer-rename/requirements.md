# Requirements Document

## Project Description (Input)
Layer名をRuntimeからServiceに変更。更に以下のように変更提案2: A-I-P-S (Service Layer) ※サフィックス変更
もし「Service」という用語自体（機能の提供という意味合い）が気に入っている場合は、フォルダー名のサフィックス（接尾辞）側を変更して重複を回避するのが実務的です。

Structure:

Plaintext

├── crates/
│   # ...
│   # --- S: Service Layer ---
│   ├── service_cli/
│   ├── service_python/
│   └── service_gateway/   # or service_api / service_daemon
Why:

service_server を service_gateway や service_api とすることで、「サービス層への入り口」というニュアンスになり、プロフェッショナルな響きになります。特にマイクロサービス文脈では Gateway が適切です。

## Requirements
<!-- Will be generated in /kiro:spec-requirements phase -->
