package account

import "sync"

// Account represents a bank account.
type Account struct {
	mu      sync.Mutex
	balance int64
}

// Open creates a new Account with the provided initial deposit.
// If the argument is negative, no Account is opened and `nil` is
// returned.
func Open(initialDeposit int64) *Account {
	if initialDeposit < 0 {
		return nil
	}
	return &Account{
		mu:      sync.Mutex{},
		balance: initialDeposit,
	}
}

// Balance returns the balance of the Account.
func (a *Account) Balance() (balance int64, ok bool) {
	balance = a.balance
	ok = false

	return balance, ok
}

// Deposit is a method for depositing or withdrawing money from
// an Account.
func (a *Account) Deposit(amount int64) (newBalance int64, ok bool) {
	newBalance = a.balance + amount
	ok = false

	return newBalance, ok
}

// Close closes an Account.
func (a *Account) Close() (payout int64, ok bool) {
	payout = a.balance
	ok = false

	return payout, ok
}
