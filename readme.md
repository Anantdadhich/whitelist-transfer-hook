# Whitelist Transfer Hook

## 🎯 What It Is

A **whitelist transfer hook** is an automated gatekeeper for Solana tokens. It checks every token transfer and only allows it if both the sender and receiver are on an approved list. Think of it like a VIP-only club for your tokens.

---

## 🔑 Key Components

### 1. **Whitelist Storage**

* Stored on-chain as a PDA (Program Derived Address)
* Holds a dynamic list of approved addresses
* Expands and shrinks automatically, with rent refunds

### 2. **Extra Account Meta List**

* Informs Token-2022 to pass the whitelist to the hook
* Created once during setup
* Essential: without it, the hook cannot function

### 3. **Transfer Hook Logic**

* Validates sender ✓
* Validates receiver ✓
* Runs automatically on every transfer
* Cannot be bypassed

---

## 💡 How It Works

```
User transfers token
    ↓
Token-2022 detects hook
    ↓
Reads extra account meta list
    ↓
Derives whitelist PDA
    ↓
Invokes your hook program
    ↓
Hook checks both parties
    ↓
✅ Both approved → Transfer succeeds
❌ Either blocked → Transfer fails
```

---

## 🛠️ Implementation (4 Steps)

### **Step 1: Initialize Whitelist**

```rust
initialize_whitelist()
→ Creates PDA with seeds: ["whitelist"]
→ Starts empty: []
→ Cost: ~0.002 SOL
```

### **Step 2: Setup Hook Metadata**

```rust
initialize_extra_account_meta_list(mint)
→ Links whitelist to Token-2022
→ Cost: ~0.002 SOL
```

### **Step 3: Add Users**

```rust
add_to_whitelist(address)
→ Expands account by 32 bytes
→ Cost: ~0.0003 SOL per address
→ Fully refundable
```

### **Step 4: Validate Transfers**

```rust
transfer_hook(amount)
→ Automatically triggered by Token-2022
→ Checks whitelist
→ Approves or rejects transfer
```

---

## 🧪 Test Results

✅ **8 Passing Tests:**

1. Initialize whitelist
2. Add  to whitelist
3. Remove from whitelist
4. Create mint with hook
5. Create token accounts
6. Initialize extra account meta
8. Re added users to whitlist
7. Transfer (validated by hook)


**Result:** All tests passed 🎉

---

## 🎯 Use Cases

* **DeFi**: KYC-only trading
* **Gaming**: Verified players only
* **Corporate**: Employee-restricted tokens
* **NFTs**: Approved marketplace enforcement
* **Private Sales**: Investor-only transfers

---

## 🚀 The Magic

Users don’t have to do anything differently — they transfer tokens normally.
Behind the scenes:

* Token-2022 detects the hook
* Gathers required accounts
* Runs validation automatically
* Enforces whitelist rules seamlessly

---

## ✨ Bottom Line

Whitelist transfer hooks turn ordinary tokens into **smart, compliant financial instruments** with built-in control and compliance. No external contracts needed — it’s all native to Token-2022.

---

