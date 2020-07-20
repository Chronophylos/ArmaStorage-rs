// Load Extension
callExtension "arma_storage"

// Print Error Codes
"arma_storage" callExtension "errorCodes"

// Open Test Storage
// result should be ["", 0, 0]
"arma_storage" callExtension ["", ["open", "test"]]

// Open alread opened Test Storage
// result should be ["Error: Storage file is open", 20, 0]
"arma_storage" callExtension ["", ["open", "test"]]

// Close Test Storage
// result should be ["", 0, 0]
"arma_storage" callExtension ["", ["close", "test"]]

// Close closed Test Storage
// result should be ["Error: Storage file is not open", 20, 0]
"arma_storage" callExtension ["", ["close", "test"]]

sleep 1

// Read closed "spam" Storage
// result should be ["Error: Storage is not open", 20, 0]
"arma_storage" callExtension ["", ["read", "spam"]]

// Write closed "spam" Storage
// result should be ["Error: Storage is not open", 20, 0]
"arma_storage" callExtension ["", ["write", "spam"]]

// Open "spam" Storage
// result should be ["", 0, 0]
"arma_storage" callExtension ["", ["open", "spam"]]

// Read nonexistent "spam" Storage
// result should be ["Error: File not found. (os error 2)", 20, 0]
"arma_storage" callExtension ["", ["read", "spam"]]

// Write "spam" Storage
// result should be ["", 0, 0]
"arma_storage" callExtension ["", ["write", "spam"]]

sleep 2

// Unload Extension
freeExtension "arma_storage"

exit
