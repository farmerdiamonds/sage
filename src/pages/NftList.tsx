import Container from '@/components/Container';
import Header from '@/components/Header';
import { MultiSelectActions } from '@/components/MultiSelectActions';
import { NftCard, NftCardList } from '@/components/NftCard';
import { NftOptions } from '@/components/NftOptions';
import { ReceiveAddress } from '@/components/ReceiveAddress';
import { Button } from '@/components/ui/button';
import {
  DropdownMenu,
  DropdownMenuContent,
  DropdownMenuGroup,
  DropdownMenuItem,
  DropdownMenuTrigger,
} from '@/components/ui/dropdown-menu';
import { useErrors } from '@/hooks/useErrors';
import { useNftParams } from '@/hooks/useNftParams';
import collectionImage from '@/images/collection.png';
import { t } from '@lingui/core/macro';
import { Trans } from '@lingui/react/macro';
import { EyeIcon, EyeOff, ImagePlusIcon, MoreVerticalIcon } from 'lucide-react';
import { useCallback, useEffect, useState } from 'react';
import { Link, useNavigate } from 'react-router-dom';
import { commands, events, NftCollectionRecord, NftRecord } from '../bindings';

export function NftList() {
  const navigate = useNavigate();

  const { addError } = useErrors();

  const [params, setParams] = useNftParams();
  const { pageSize, page, view, showHidden, query } = params;

  const [nfts, setNfts] = useState<NftRecord[]>([]);
  const [collections, setCollections] = useState<NftCollectionRecord[]>([]);
  const [multiSelect, setMultiSelect] = useState(false);
  const [selected, setSelected] = useState<string[]>([]);
  const [isLoading, setIsLoading] = useState(false);

  const updateNfts = useCallback(
    async (page: number) => {
      setIsLoading(true);
      try {
        if (view === 'name' || view === 'recent') {
          await commands
            .getNfts({
              collection_id: null,
              did_id: null,
              name: query || null,
              offset: (page - 1) * pageSize,
              limit: pageSize,
              sort_mode: view,
              include_hidden: showHidden,
            })
            .then((data) => setNfts(data.nfts))
            .catch(addError);
        } else if (view === 'collection') {
          await commands
            .getNftCollections({
              offset: (page - 1) * pageSize,
              limit: pageSize,
              include_hidden: showHidden,
            })
            .then((data) => setCollections(data.collections))
            .catch(addError);
        }
      } finally {
        setIsLoading(false);
      }
    },
    [pageSize, showHidden, view, query, addError],
  );

  useEffect(() => {
    updateNfts(1);
  }, [updateNfts]);

  useEffect(() => {
    const unlisten = events.syncEvent.listen((event) => {
      const type = event.payload.type;

      if (
        type === 'coin_state' ||
        type === 'puzzle_batch_synced' ||
        type === 'nft_data'
      ) {
        updateNfts(page);
      }
    });

    return () => {
      unlisten.then((u) => u());
    };
  }, [updateNfts, page]);

  useEffect(() => {
    updateNfts(page);
  }, [updateNfts, page]);

  return (
    <>
      <Header title={<Trans>NFTs</Trans>}>
        <ReceiveAddress />
      </Header>

      <Container>
        <Button onClick={() => navigate('/nfts/mint')}>
          <ImagePlusIcon className='h-4 w-4 mr-2' /> <Trans>Mint NFT</Trans>
        </Button>

        <NftOptions
          params={params}
          setParams={setParams}
          multiSelect={multiSelect}
          setMultiSelect={(value) => {
            setMultiSelect(value);
            setSelected([]);
          }}
          className='mt-4'
          isLoading={isLoading}
          canLoadMore={nfts.length === pageSize}
        />

        <NftCardList>
          {view === 'collection' ? (
            <>
              {collections.map((col, i) => (
                <Collection
                  col={col}
                  key={i}
                  updateNfts={() => updateNfts(page)}
                />
              ))}
              {nfts.length < pageSize && (
                <Collection
                  col={{
                    name: 'Uncategorized NFTs',
                    icon: '',
                    did_id: 'Miscellaneous',
                    metadata_collection_id: 'Uncategorized',
                    collection_id: 'No collection',
                    visible: true,
                  }}
                  updateNfts={() => updateNfts(page)}
                />
              )}
            </>
          ) : (
            nfts.map((nft, i) => (
              <NftCard
                nft={nft}
                key={i}
                updateNfts={() => updateNfts(page)}
                selectionState={
                  multiSelect
                    ? [
                        selected.includes(nft.launcher_id),
                        (value) => {
                          if (value && !selected.includes(nft.launcher_id)) {
                            setSelected([...selected, nft.launcher_id]);
                          } else if (
                            !value &&
                            selected.includes(nft.launcher_id)
                          ) {
                            setSelected(
                              selected.filter((id) => id !== nft.launcher_id),
                            );
                          }
                        },
                      ]
                    : null
                }
              />
            ))
          )}
        </NftCardList>
      </Container>

      {selected.length > 0 && (
        <MultiSelectActions
          selected={selected}
          onConfirm={() => {
            setSelected([]);
            setMultiSelect(false);
          }}
        />
      )}
    </>
  );
}

interface CollectionProps {
  col: NftCollectionRecord;
  updateNfts: () => void;
}

function Collection({ col }: CollectionProps) {
  return (
    <>
      <Link
        to={`/collections/${col.collection_id}`}
        className={`group${`${!col.visible ? ' opacity-50 grayscale' : ''}`} border border-neutral-200 rounded-lg dark:border-neutral-800`}
      >
        <div className='overflow-hidden rounded-t-lg relative'>
          <img
            alt={col.name ?? t`Unnamed`}
            loading='lazy'
            width='150'
            height='150'
            className='h-auto w-auto object-cover transition-all group-hover:scale-105 aspect-square color-[transparent]'
            src={collectionImage}
          />
        </div>
        <div className='border-t bg-white text-neutral-950 shadow  dark:bg-neutral-900 dark:text-neutral-50 text-md flex items-center justify-between rounded-b-lg p-2 pl-3'>
          <span className='truncate'>
            <span className='font-medium leading-none truncate'>
              {col.name ?? <Trans>Unnamed</Trans>}
            </span>
            {col.collection_id && (
              <p className='text-xs text-muted-foreground truncate'>
                {col.collection_id}
              </p>
            )}
          </span>

          <DropdownMenu>
            <DropdownMenuTrigger asChild>
              <Button variant='ghost' size='icon'>
                <MoreVerticalIcon className='h-5 w-5' />
              </Button>
            </DropdownMenuTrigger>
            <DropdownMenuContent align='end'>
              <DropdownMenuGroup>
                <DropdownMenuItem
                  className='cursor-pointer'
                  onClick={(e) => {
                    e.stopPropagation();
                    // toggleVisibility();
                  }}
                >
                  {col.visible ? (
                    <EyeOff className='mr-2 h-4 w-4' />
                  ) : (
                    <EyeIcon className='mr-2 h-4 w-4' />
                  )}
                  <span>{col.visible ? t`Hide` : t`Show`}</span>
                </DropdownMenuItem>
              </DropdownMenuGroup>
            </DropdownMenuContent>
          </DropdownMenu>
        </div>
      </Link>
    </>
  );
}
