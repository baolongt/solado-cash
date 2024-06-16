/**
  create component account deposit withdrawal token
  2 tab deposit and withdrawal
  deposit tabs - form Token select 2 options SOL and USDC , Amount input number, button submit action show console log with value
  withdrawal tabs - form Note input textarea auto expand height , Recipient Address input text, button submit action show console log with value
  daisyUI for styling
  when submit button it will show loading
  use useMutation  from "@tanstack/react-query" for loading state
*/
import React, { useState } from 'react';

const AccountDepositWithdrawalToken: React.FC = () => {
  const [tab, setTab] = useState<'deposit' | 'withdrawal'>('deposit');
  const [token, setToken] = useState<'SOL' | 'USDC'>('SOL');
  const [amount, setAmount] = useState<number>(0);
  const [note, setNote] = useState<string>('');
  const [recipientAddress, setRecipientAddress] = useState<string>('');
  const [isLoading, setIsLoading] = useState<boolean>(false);

  const handleTabChange = (selectedTab: 'deposit' | 'withdrawal') => {
    setTab(selectedTab);
  };

  const handleTokenChange = (selectedToken: 'SOL' | 'USDC') => {
    setToken(selectedToken);
  };

  const handleAmountChange = (event: React.ChangeEvent<HTMLInputElement>) => {
    setAmount(Number(event.target.value));
  };

  const handleNoteChange = (event: React.ChangeEvent<HTMLTextAreaElement>) => {
    setNote(event.target.value);
  };

  const handleRecipientAddressChange = (event: React.ChangeEvent<HTMLInputElement>) => {
    setRecipientAddress(event.target.value);
  };

  const handleSubmit = () => {
    setIsLoading(true);
    console.log('Token:', token);
    console.log('Amount:', amount);
    console.log('Note:', note);
    console.log('Recipient Address:', recipientAddress);
    setTimeout(() => {
      setIsLoading(false);
    }, 2000);
  };

  return (
    <div className="flex flex-col">
      <div className="flex justify-center">
        <button
          className={`px-4 py-2 rounded-tl-lg rounded-tr-lg ${
            tab === 'deposit' ? 'bg-blue-500 text-white' : 'bg-gray-200 text-gray-600'
          }`}
          onClick={() => handleTabChange('deposit')}
        >
          Deposit
        </button>
        <button
          className={`px-4 py-2 rounded-tl-lg rounded-tr-lg ${
            tab === 'withdrawal' ? 'bg-blue-500 text-white' : 'bg-gray-200 text-gray-600'
          }`}
          onClick={() => handleTabChange('withdrawal')}
        >
          Withdrawal
        </button>
      </div>
      <div className="p-4">
        {tab === 'deposit' ? (
          <div>
            <div className="mb-4">
              <label className="block mb-2 font-bold text-gray-700" htmlFor="token">
                Token
              </label>
              <select
                className="w-full px-3 py-2 border border-gray-300 rounded-md focus:outline-none focus:ring focus:ring-blue-500"
                id="token"
                value={token}
                onChange={(e) => handleTokenChange(e.target.value as 'SOL' | 'USDC')}
              >
                <option value="SOL">SOL</option>
                <option value="USDC">USDC</option>
              </select>
            </div>
            <div className="mb-4">
              <label className="block mb-2 font-bold text-gray-700" htmlFor="amount">
                Amount
              </label>
              <input
                className="w-full px-3 py-2 border border-gray-300 rounded-md focus:outline-none focus:ring focus:ring-blue-500"
                type="number"
                id="amount"
                value={amount}
                onChange={handleAmountChange}
              />
            </div>
            <button
              className="w-full px-4 py-2 text-white bg-blue-500 rounded-md hover:bg-blue-600 focus:outline-none focus:ring focus:ring-blue-500"
              onClick={handleSubmit}
            >
              {isLoading ? 'Loading...' : 'Submit'}
            </button>
          </div>
        ) : (
          <div>
            <div className="mb-4">
              <label className="block mb-2 font-bold text-gray-700" htmlFor="note">
                Note
              </label>
              <textarea
                className="w-full px-3 py-2 border border-gray-300 rounded-md focus:outline-none focus:ring focus:ring-blue-500"
                id="note"
                value={note}
                onChange={handleNoteChange}
              />
            </div>
            <div className="mb-4">
              <label className="block mb-2 font-bold text-gray-700" htmlFor="recipientAddress">
                Recipient Address
              </label>
              <input
                className="w-full px-3 py-2 border border-gray-300 rounded-md focus:outline-none focus:ring focus:ring-blue-500"
                type="text"
                id="recipientAddress"
                value={recipientAddress}
                onChange={handleRecipientAddressChange}
              />
            </div>
            <button
              className="w-full px-4 py-2 text-white bg-blue-500 rounded-md hover:bg-blue-600 focus:outline-none focus:ring focus:ring-blue-500"
              onClick={handleSubmit}
            >
              {isLoading ? 'Loading...' : 'Submit'}
            </button>
          </div>
        )}
      </div>
    </div>
  );
};

export default AccountDepositWithdrawalToken;

