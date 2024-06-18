
    'use client';


import { programId, getTodoProgram } from '@solado/anchor';
import { Program } from '@coral-xyz/anchor';
import { useConnection } from '@solana/wallet-adapter-react';
import { Keypair } from '@solana/web3.js';
import { useMutation, useQuery } from '@tanstack/react-query';
import toast from 'react-hot-toast';
import { useCluster } from '../cluster/cluster-data-access';
import { useAnchorProvider } from '../solana/solana-provider';
import { useTransactionToast } from '../ui/ui-layout';

export function useTodoProgram() {
  const { connection } = useConnection();
  const { cluster } = useCluster();
  const transactionToast = useTransactionToast();
  const provider = useAnchorProvider();
  const program = getTodoProgram(provider);

  const getProgramAccount = useQuery({
    queryKey: ['get-program-account', { cluster }],
    queryFn: () => connection.getParsedAccountInfo(programId),
  });

  const greet = useMutation({
    mutationKey: ['todo', 'greet', { cluster }],
    mutationFn: (keypair: Keypair) => program.methods.greet().rpc(),
    onSuccess: (signature) => {
      transactionToast(signature);
    },
    onError: () => toast.error('Failed to run program'),
  });

  return {
    program,
    programId,
    getProgramAccount,
    greet,
  };
}
