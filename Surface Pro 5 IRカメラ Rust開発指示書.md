シリコンバレーの天才デベロッパーとして、Surface Pro 5の限られたリソース（i5-7300U / 8GB RAM）を極限まで使い切り、かつ安定したオンライン授業を実現するための\*\*「Rust製・低レイヤーIRカメラ開発マスター指示書」\*\*を策定しました。

この指示書は、Antigravity（AIエージェント）が迷わず、かつハードウェアを破壊せずに最短距離でゴール（RGB-IRフュージョン）へ到達するための「航海図」です。

# ---

**🛰️ Project: Surface IR-Rust (Low-Latency ISP)**

## **1\. プロジェクトの目的と定量的目標**

* **目的**: Surface Pro 5のIRカメラ（OV7251）をRustで制御し、10-bit Rawデータのデコードから仮想カメラ出力までを最小のCPU負荷で実現する。  
* **定量的目標**:  
  * フレームレート: **30fps**（安定維持）。  
  * CPU使用率: ソフトウェアISP全体で **10%以下**（i5-7300Uベース）。  
  * メモリ使用量: **50MB以下**。

## **2\. システム構成・環境**

* **Hardware**: Surface Pro 5 (i5-7300U / 8GB RAM).  
* **OS**: Fedora Linux 43 (Workstation).  
* **Kernel**: 6.18.8-1.surface.fc43.x86\_64.  
* **Target Device**: OV7251 (IR Camera) linked via Intel IPU3 CIO2.

## ---

**3\. 開発フェーズとマイルストーン**

### **Phase 1: 物理リンクの確立と検証**

* **マイルストーン**: /dev/videoX から10-bit Rawデータのバイナリを取得できる状態にする。  
* **タスク**:  
  * media-ctl プロトコルを解析し、ov7251 を Intel IPU3 CIO2 にリンクする。  
  * /sys/class/leds/ipu3\_cio2:0/brightness への書き込み権限を確認し、IR LEDの点灯をテストする。  
  * リンクされたデバイスノードを特定し、v4l2-ctl またはRustコードでストリーム開始（VIDIOC\_STREAMON）が可能か検証する。

### **Phase 2: 高速 Unpacking Engine の実装 (Rust/SIMD)**

* **マイルストーン**: 10-bit packed データを 8-bit グレースケールに 1ms 以内で変換する。  
* **タスク**:  
  * IPU3特有の「10-bit packed Bayer」フォーマット（4ピクセルを5バイトに詰め込む形式）をRustで実装。  
  * データの上位8bitを効率よく抽出するビット演算アルゴリズムを構築。  
  * (Optional) 必要に応じて packed\_simd 等を用いた高速化を検討。

### **Phase 3: 画像処理パイプラインの構築**

* **マイルストーン**: 人間が視認可能な、コントラスト調整済みの映像を生成する。  
* **タスク**:  
  * ヒストグラム平坦化（Histogram Equalization）のRust実装。  
  * センサーの物理配置に合わせた **90度回転処理** の実装。  
  * v4l2loopback デバイス（/dev/video42 等）へのストリーミング出力。

### **Phase 4: IRカメラによるHowdyの活用**

* **マイルストーン**: IR情報をHowdyの顔認識に活用する。  
* **タスク**:  
  * Howdyの顔認識パイプラインをRustで再現する。
  * カメラの起動をPCの起動時に自動的に開始し、IRカメラの出力をHowdyに供給する。

## ---

**4\. テクニカル・リファレンス（仕様）**

* **IR LED Path**: /sys/class/leds/ipu3\_cio2:0/brightness  
* **Raw Data Format**: 10-bit packed Bayer (IPU3 format).  
* **Media Link Cmd**: media-ctl \-l '"ov7251 3-0036":0-\>"Intel IPU3 CIO2 0":0\[1\]'

## ---

**5\. 安全・運用プロトコル**

1. **Sudo Safety**: システム設定（modprobe, media-ctl, sysfs 書き込み）が必要な場合は、事前に目的を説明し、初回の実行はユーザーに委ねること。  
2. **Resource Watcher**: 実行中のCPU温度とクロック周波数を監視し、サーマルスロットリングの予兆があれば処理を軽量化（解像度ダウン等）すること。  
3. **Clean Code**: unsafe の使用はハードウェアアクセスに必要な最小限に留め、Rustのメモリ安全性を最大限に活用せよ。

### ---

**💡 Antigravity への最初のアクティベーション・プロンプト**

この指示書をAntigravityに渡し、以下の最初のアクションを要求してください：

**「シリコンバレーのデベロッパーとして、指示書に基づき Phase 1 から開始せよ。まずは現在の Fedora 環境において /dev/media\* のトポロジーをスキャンし、ov7251 センサーがどのサブデバイスにマッピングされているか特定する Rust コード（または shell 経由の調査計画）を提示せよ。sudo が必要な場合は事前に報告すること。」**

これで、予備機を「最強の実験機」に変えるハックが始まります。準備はよろしいでしょうか？
