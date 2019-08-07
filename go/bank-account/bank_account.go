package account

import "sync"

// Account represents a bank account.
type Account struct {
	sync.Mutex

	balance int64
	open    bool
}

// Open creates a new Account with the provided initial deposit.
// If the argument is negative, no Account is opened and `nil` is
// returned.
func Open(initialDeposit int64) *Account {
	if initialDeposit < 0 {
		return nil
	}
	return &Account{
		balance: initialDeposit,
		open:    true,
	}
}

// Balance returns the balance of the Account.
func (a *Account) Balance() (int64, bool) {
	a.Lock()
	defer a.Unlock()

	if !a.open {
		return 0, false
	}

	return a.balance, true
}

// Deposit is a method for depositing or withdrawing money from
// an Account.
func (a *Account) Deposit(amount int64) (int64, bool) {
	a.Lock()
	defer a.Unlock()
	if !a.open {
		return 0, false
	}

	a.balance += amount
	if a.balance < 0 {
		return 0, false
	}

	return a.balance, true
}

// Close closes an Account.
func (a *Account) Close() (int64, bool) {
	a.Lock()
	defer a.Unlock()
	if !a.open {
		return 0, false
	}

	payout := a.balance
	a.balance = 0
	a.open = false

	return payout, true
}
