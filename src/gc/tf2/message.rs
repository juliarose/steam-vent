use crate::gc::GCMessage;
use super::proto::{
    econ_gcmessages,
    base_gcmessages,
    gcsdk_gcmessages,
    tf_gcmessages,
};

macro_rules! gc_msg {
    ($req:path) => {
        impl GCMessage for $req {}
    };
}

gc_msg!(gcsdk_gcmessages::CMsgSOIDOwner);
gc_msg!(gcsdk_gcmessages::CMsgSOSingleObject);
gc_msg!(gcsdk_gcmessages::CMsgSOMultipleObjects);
gc_msg!(gcsdk_gcmessages::CMsgSOMultipleObjects_SingleObject);
gc_msg!(gcsdk_gcmessages::CMsgSOCacheSubscribed);
gc_msg!(gcsdk_gcmessages::CMsgSOCacheSubscribed_SubscribedType);
gc_msg!(gcsdk_gcmessages::CMsgSOCacheSubscribedUpToDate);
gc_msg!(gcsdk_gcmessages::CMsgSOCacheUnsubscribed);
gc_msg!(gcsdk_gcmessages::CMsgSOCacheSubscriptionCheck);
gc_msg!(gcsdk_gcmessages::CMsgSOCacheSubscriptionRefresh);
gc_msg!(gcsdk_gcmessages::CMsgSOCacheVersion);
gc_msg!(gcsdk_gcmessages::CMsgGCMultiplexMessage);
gc_msg!(gcsdk_gcmessages::CGCToGCMsgMasterAck);
gc_msg!(gcsdk_gcmessages::CGCToGCMsgMasterAck_Response);
gc_msg!(gcsdk_gcmessages::CGCToGCMsgMasterStartupComplete);
gc_msg!(gcsdk_gcmessages::CGCToGCMsgMasterStartupComplete_GCInfo);
gc_msg!(gcsdk_gcmessages::CGCToGCMsgRouted);
gc_msg!(gcsdk_gcmessages::CGCToGCMsgRoutedReply);
gc_msg!(gcsdk_gcmessages::CMsgGCUpdateSubGCSessionInfo);
gc_msg!(gcsdk_gcmessages::CMsgGCUpdateSubGCSessionInfo_CMsgUpdate);
gc_msg!(gcsdk_gcmessages::CMsgGCRequestSubGCSessionInfo);
gc_msg!(gcsdk_gcmessages::CMsgGCRequestSubGCSessionInfoResponse);
gc_msg!(gcsdk_gcmessages::CMsgGCToGCIncrementRecruitmentLevel);
gc_msg!(gcsdk_gcmessages::CMsgSOCacheHaveVersion);
gc_msg!(gcsdk_gcmessages::CMsgConnectionStatus);
gc_msg!(gcsdk_gcmessages::CMsgGCToGCSOCacheSubscribe);
gc_msg!(gcsdk_gcmessages::CMsgGCToGCSOCacheSubscribe_CMsgHaveVersions);
gc_msg!(gcsdk_gcmessages::CMsgGCToGCSOCacheUnsubscribe);
gc_msg!(gcsdk_gcmessages::CMsgGCClientPing);
gc_msg!(base_gcmessages::CGCStorePurchaseInit_LineItem);
gc_msg!(base_gcmessages::CMsgGCStorePurchaseInit);
gc_msg!(base_gcmessages::CMsgGCStorePurchaseInitResponse);
gc_msg!(base_gcmessages::CMsgSystemBroadcast);
gc_msg!(base_gcmessages::CMsgClientHello);
gc_msg!(base_gcmessages::CMsgServerHello);
gc_msg!(base_gcmessages::CMsgClientWelcome);
gc_msg!(base_gcmessages::CMsgServerWelcome);
gc_msg!(base_gcmessages::CMsgClientGoodbye);
gc_msg!(base_gcmessages::CMsgServerGoodbye);
gc_msg!(base_gcmessages::CMsgServerAvailable);
gc_msg!(base_gcmessages::CMsgLANServerAvailable);
gc_msg!(base_gcmessages::CSOEconGameAccountClient);
gc_msg!(base_gcmessages::CSOItemCriteriaCondition);
gc_msg!(base_gcmessages::CSOItemCriteria);
gc_msg!(base_gcmessages::CSOItemRecipe);
gc_msg!(base_gcmessages::CMsgDevNewItemRequest);
gc_msg!(base_gcmessages::CMsgDevDebugRollLootRequest);
gc_msg!(base_gcmessages::CMsgIncrementKillCountAttribute);
gc_msg!(base_gcmessages::CMsgIncrementKillCountAttribute_Multiple);
gc_msg!(base_gcmessages::CMsgTrackUniquePlayerPairEvent);
gc_msg!(base_gcmessages::CMsgApplyStrangeCountTransfer);
gc_msg!(base_gcmessages::CMsgApplyStrangePart);
gc_msg!(base_gcmessages::CMsgApplyStrangeRestriction);
gc_msg!(base_gcmessages::CMsgApplyUpgradeCard);
gc_msg!(base_gcmessages::CSOEconItemAttribute);
gc_msg!(base_gcmessages::CSOEconItemEquipped);
gc_msg!(base_gcmessages::CSOEconItem);
gc_msg!(base_gcmessages::CMsgAdjustItemEquippedState);
gc_msg!(base_gcmessages::CMsgSortItems);
gc_msg!(base_gcmessages::CSOEconClaimCode);
gc_msg!(base_gcmessages::CMsgStoreGetUserData);
gc_msg!(base_gcmessages::CMsgStoreGetUserDataResponse);
gc_msg!(base_gcmessages::CMsgUpdateItemSchema);
gc_msg!(base_gcmessages::CMsgGCError);
gc_msg!(base_gcmessages::CMsgRequestInventoryRefresh);
gc_msg!(base_gcmessages::CMsgConVarValue);
gc_msg!(base_gcmessages::CMsgReplicateConVars);
gc_msg!(base_gcmessages::CMsgUseItem);
gc_msg!(base_gcmessages::CMsgReplayUploadedToYouTube);
gc_msg!(base_gcmessages::CMsgConsumableExhausted);
gc_msg!(base_gcmessages::CMsgItemAcknowledged);
gc_msg!(base_gcmessages::CMsgSetPresetItemPosition);
gc_msg!(base_gcmessages::CMsgSetItemPositions);
gc_msg!(base_gcmessages::CMsgSetItemPositions_ItemPosition);
gc_msg!(base_gcmessages::CSOEconItemPresetInstance);
gc_msg!(base_gcmessages::CMsgSelectPresetForClass);
gc_msg!(base_gcmessages::CSOClassPresetClientData);
gc_msg!(base_gcmessages::CMsgGCReportAbuse);
gc_msg!(base_gcmessages::CMsgGCReportAbuseResponse);
gc_msg!(base_gcmessages::CMsgGCNameItemNotification);
gc_msg!(base_gcmessages::CMsgGCClientDisplayNotification);
gc_msg!(base_gcmessages::CMsgGCShowItemsPickedUp);
gc_msg!(base_gcmessages::CMsgUpdatePeriodicEvent);
gc_msg!(base_gcmessages::CMsgGCIncrementKillCountResponse);
gc_msg!(base_gcmessages::CMsgGCRemoveStrangePart);
gc_msg!(base_gcmessages::CMsgGCRemoveUpgradeCard);
gc_msg!(base_gcmessages::CMsgGCRemoveCustomizationAttributeSimple);
gc_msg!(base_gcmessages::CMsgGCResetStrangeScores);
gc_msg!(base_gcmessages::CMsgGCItemPreviewItemBoughtNotification);
gc_msg!(base_gcmessages::CMsgGCStorePurchaseCancel);
gc_msg!(base_gcmessages::CMsgGCStorePurchaseCancelResponse);
gc_msg!(base_gcmessages::CMsgGCStorePurchaseFinalize);
gc_msg!(base_gcmessages::CMsgGCStorePurchaseFinalizeResponse);
gc_msg!(base_gcmessages::CMsgGCBannedWordListRequest);
gc_msg!(base_gcmessages::CMsgGCGiftedItems);
gc_msg!(base_gcmessages::CMsgGCCollectItem);
gc_msg!(base_gcmessages::CMsgGCClientMarketDataRequest);
gc_msg!(base_gcmessages::CMsgGCClientMarketDataEntry);
gc_msg!(base_gcmessages::CMsgGCClientMarketData);
gc_msg!(base_gcmessages::CMsgApplyToolToItem);
gc_msg!(base_gcmessages::CMsgApplyToolToBaseItem);
gc_msg!(base_gcmessages::CMsgRecipeComponent);
gc_msg!(base_gcmessages::CMsgFulfillDynamicRecipeComponent);
gc_msg!(base_gcmessages::CMsgSetItemEffectVerticalOffset);
gc_msg!(base_gcmessages::CMsgSetHatEffectUseHeadOrigin);
gc_msg!(base_gcmessages::CMsgDeliverGiftResponseGiver);
gc_msg!(base_gcmessages::CSOEconGameAccountForGameServers);
gc_msg!(base_gcmessages::CWorkshop_PopulateItemDescriptions_Request);
gc_msg!(base_gcmessages::CWorkshop_PopulateItemDescriptions_Request_SingleItemDescription);
gc_msg!(base_gcmessages::CWorkshop_PopulateItemDescriptions_Request_ItemDescriptionsLanguageBlock);
gc_msg!(base_gcmessages::CWorkshop_GetContributors_Request);
gc_msg!(base_gcmessages::CWorkshop_GetContributors_Response);
gc_msg!(base_gcmessages::CWorkshop_SetItemPaymentRules_Request);
gc_msg!(base_gcmessages::CWorkshop_SetItemPaymentRules_Request_WorkshopItemPaymentRule);
gc_msg!(base_gcmessages::CWorkshop_SetItemPaymentRules_Request_PartnerItemPaymentRule);
gc_msg!(base_gcmessages::CWorkshop_SetItemPaymentRules_Response);
gc_msg!(econ_gcmessages::CMsgApplyAutograph);
gc_msg!(econ_gcmessages::CMsgEconPlayerStrangeCountAdjustment);
gc_msg!(econ_gcmessages::CMsgEconPlayerStrangeCountAdjustment_CStrangeCountAdjustment);
gc_msg!(econ_gcmessages::CMsgRequestItemPurgatory_FinalizePurchase);
gc_msg!(econ_gcmessages::CMsgRequestItemPurgatory_FinalizePurchaseResponse);
gc_msg!(econ_gcmessages::CMsgRequestItemPurgatory_RefundPurchase);
gc_msg!(econ_gcmessages::CMsgRequestItemPurgatory_RefundPurchaseResponse);
gc_msg!(econ_gcmessages::CMsgCraftingResponse);
gc_msg!(econ_gcmessages::CMsgGCRequestStoreSalesData);
gc_msg!(econ_gcmessages::CMsgGCRequestStoreSalesDataResponse);
gc_msg!(econ_gcmessages::CMsgGCRequestStoreSalesDataResponse_Price);
gc_msg!(econ_gcmessages::CMsgGCRequestStoreSalesDataUpToDateResponse);
gc_msg!(econ_gcmessages::CMsgGCToGCPingRequest);
gc_msg!(econ_gcmessages::CMsgGCToGCPingResponse);
gc_msg!(econ_gcmessages::CMsgGCToGCGetUserSessionServer);
gc_msg!(econ_gcmessages::CMsgGCToGCGetUserSessionServerResponse);
gc_msg!(econ_gcmessages::CMsgGCToGCGetUserServerMembers);
gc_msg!(econ_gcmessages::CMsgGCToGCGetUserServerMembersResponse);
gc_msg!(econ_gcmessages::CMsgLookupMultipleAccountNames);
gc_msg!(econ_gcmessages::CMsgLookupMultipleAccountNamesResponse);
gc_msg!(econ_gcmessages::CMsgLookupMultipleAccountNamesResponse_Account);
gc_msg!(econ_gcmessages::CMsgGCToGCGrantSelfMadeItemToAccount);
gc_msg!(econ_gcmessages::CMsgGCToGCThankedByNewUser);
gc_msg!(econ_gcmessages::CMsgGCShuffleCrateContents);
gc_msg!(econ_gcmessages::CMsgGCQuestObjective_Progress);
gc_msg!(econ_gcmessages::CMsgGCQuestObjective_PointsChange);
gc_msg!(econ_gcmessages::CMsgGCQuestComplete_Request);
gc_msg!(econ_gcmessages::CMsgGCQuestCompleted);
gc_msg!(econ_gcmessages::CMsgGCQuestObjective_RequestLoanerItems);
gc_msg!(econ_gcmessages::CMsgGCQuestObjective_RequestLoanerResponse);
gc_msg!(econ_gcmessages::CMsgCraftCollectionUpgrade);
gc_msg!(econ_gcmessages::CMsgCraftHalloweenOffering);
gc_msg!(econ_gcmessages::CMsgCraftCommonStatClock);
gc_msg!(econ_gcmessages::CMsgGCQuestDiscard_Request);
gc_msg!(tf_gcmessages::CMsgTFGoldenWrenchBroadcast);
gc_msg!(tf_gcmessages::CMsgTFSaxxyBroadcast);
gc_msg!(tf_gcmessages::CMsgGCTFSpecificItemBroadcast);
gc_msg!(tf_gcmessages::CMsgTFWorldStatus);
gc_msg!(tf_gcmessages::CSOTFDuelSummary);
gc_msg!(tf_gcmessages::CSOTFMapContribution);
gc_msg!(tf_gcmessages::CMsgTFVoteKickBanPlayer);
gc_msg!(tf_gcmessages::CMsgTFVoteKickBanPlayerResult);
gc_msg!(tf_gcmessages::CMsgTFFreeTrialChooseMostHelpfulFriend);
gc_msg!(tf_gcmessages::CMsgTFRequestTF2Friends);
gc_msg!(tf_gcmessages::CMsgTFRequestTF2FriendsResponse);
gc_msg!(tf_gcmessages::CSOTFPlayerInfo);
gc_msg!(tf_gcmessages::CMsgTFThankedBySomeone);
gc_msg!(tf_gcmessages::CMsgTFThankedSomeone);
gc_msg!(tf_gcmessages::CMsgTFFreeTrialConvertedToPremium);
gc_msg!(tf_gcmessages::CMsgSaxxyAwarded);
gc_msg!(tf_gcmessages::CMsgReplaySubmitContestEntry);
gc_msg!(tf_gcmessages::CMsgReplaySubmitContestEntryResponse);
gc_msg!(tf_gcmessages::CReplayCachedContestData);
gc_msg!(tf_gcmessages::CMsgTFCoaching_AddToCoaches);
gc_msg!(tf_gcmessages::CMsgTFCoaching_RemoveFromCoaches);
gc_msg!(tf_gcmessages::CMsgTFCoaching_FindCoach);
gc_msg!(tf_gcmessages::CMsgTFCoaching_FindCoachResponse);
gc_msg!(tf_gcmessages::CMsgTFCoaching_AskCoach);
gc_msg!(tf_gcmessages::CMsgTFCoaching_AskCoachResponse);
gc_msg!(tf_gcmessages::CMsgTFCoaching_CoachJoinGame);
gc_msg!(tf_gcmessages::CMsgTFCoaching_CoachJoining);
gc_msg!(tf_gcmessages::CMsgTFCoaching_CoachJoined);
gc_msg!(tf_gcmessages::CMsgTFCoaching_LikeCurrentCoach);
gc_msg!(tf_gcmessages::CMsgTFCoaching_RemoveCurrentCoach);
gc_msg!(tf_gcmessages::CMsgTFQuickplay_ScoreServers);
gc_msg!(tf_gcmessages::CMsgTFQuickplay_ScoreServers_ServerInfo);
gc_msg!(tf_gcmessages::CMsgTFQuickplay_ScoreServersResponse);
gc_msg!(tf_gcmessages::CMsgTFQuickplay_ScoreServersResponse_ServerInfo);
gc_msg!(tf_gcmessages::CMsgTFQuickplay_PlayerJoining);
gc_msg!(tf_gcmessages::CMsgGC_GameServer_LevelInfo);
gc_msg!(tf_gcmessages::CMsgGC_GameServer_AuthChallenge);
gc_msg!(tf_gcmessages::CMsgGC_GameServer_AuthResult);
gc_msg!(tf_gcmessages::CMsgGC_GameServer_AuthChallengeResponse);
gc_msg!(tf_gcmessages::CMsgGC_GameServer_CreateIdentity);
gc_msg!(tf_gcmessages::CMsgGC_GameServer_CreateIdentityResponse);
gc_msg!(tf_gcmessages::CMsgGC_GameServer_List);
gc_msg!(tf_gcmessages::CMsgGC_GameServer_ListResponse);
gc_msg!(tf_gcmessages::CMsgGC_GameServer_ListResponse_GameServerIdentity);
gc_msg!(tf_gcmessages::CMsgGC_GameServer_ResetIdentity);
gc_msg!(tf_gcmessages::CMsgGC_GameServer_ResetIdentityResponse);
gc_msg!(tf_gcmessages::CMsgGC_GameServer_AckPolicy);
gc_msg!(tf_gcmessages::CMsgGC_GameServer_AckPolicyResponse);
gc_msg!(tf_gcmessages::CMsgGC_Client_UseServerModificationItem);
gc_msg!(tf_gcmessages::CMsgGC_Client_UseServerModificationItem_Response);
gc_msg!(tf_gcmessages::CMsgGC_GameServer_UseServerModificationItem);
gc_msg!(tf_gcmessages::CMsgGC_GameServer_UseServerModificationItem_Response);
gc_msg!(tf_gcmessages::CMsgGC_GameServer_ServerModificationItemExpired);
gc_msg!(tf_gcmessages::CMsgGC_GameServer_ServerModificationItem);
gc_msg!(tf_gcmessages::CMsgGC_Halloween_ReservedItem);
gc_msg!(tf_gcmessages::CMsgGC_Halloween_GrantItem);
gc_msg!(tf_gcmessages::CMsgGC_Halloween_GrantItemResponse);
gc_msg!(tf_gcmessages::CMsgGC_Halloween_ItemClaimed);
gc_msg!(tf_gcmessages::CMsgGC_PickupItemEligibility_Query);
gc_msg!(tf_gcmessages::CMsgGC_PickupItemEligibility_QueryResponse);
gc_msg!(tf_gcmessages::CSOTFPartyMember);
gc_msg!(tf_gcmessages::CSOTFPartyMember_Activity);
gc_msg!(tf_gcmessages::TFPendingPartyMember);
gc_msg!(tf_gcmessages::TFSyncedMMUIState);
gc_msg!(tf_gcmessages::CTFGroupMatchCriteriaProto);
gc_msg!(tf_gcmessages::CTFCasualMatchCriteria);
gc_msg!(tf_gcmessages::CTFPerPlayerMatchCriteriaProto);
gc_msg!(tf_gcmessages::CTFPartyOptions);
gc_msg!(tf_gcmessages::CMsgPartySetOptions);
gc_msg!(tf_gcmessages::CMsgPartySetOptionsResponse);
gc_msg!(tf_gcmessages::CMsgPartyQueueForMatch);
gc_msg!(tf_gcmessages::CMsgPartyQueueForMatchResponse);
gc_msg!(tf_gcmessages::CMsgPartyQueueForStandby);
gc_msg!(tf_gcmessages::CMsgPartyQueueForStandbyResponse);
gc_msg!(tf_gcmessages::CMsgPartyRemoveFromQueue);
gc_msg!(tf_gcmessages::CMsgPartyRemoveFromQueueResponse);
gc_msg!(tf_gcmessages::CMsgPartyRemoveFromStandbyQueue);
gc_msg!(tf_gcmessages::CMsgPartyRemoveFromStandbyQueueResponse);
gc_msg!(tf_gcmessages::CMsgPartyInvitePlayer);
gc_msg!(tf_gcmessages::CMsgPartyRequestJoinPlayer);
gc_msg!(tf_gcmessages::CMsgPartyClearPendingPlayer);
gc_msg!(tf_gcmessages::CMsgPartyClearPendingPlayerResponse);
gc_msg!(tf_gcmessages::CMsgPartyClearOtherPartyRequest);
gc_msg!(tf_gcmessages::CMsgPartyClearOtherPartyRequestResponse);
gc_msg!(tf_gcmessages::CMsgPartyPromoteToLeader);
gc_msg!(tf_gcmessages::CMsgPartyKickMember);
gc_msg!(tf_gcmessages::CMsgPartySendChat);
gc_msg!(tf_gcmessages::CMsgPartyChatMsg);
gc_msg!(tf_gcmessages::CSOTFParty);
gc_msg!(tf_gcmessages::CSOTFParty_QueueEntry);
gc_msg!(tf_gcmessages::CSOTFPartyInvite);
gc_msg!(tf_gcmessages::CSOTFPartyInvite_PartyMember);
gc_msg!(tf_gcmessages::CTFLobbyPlayerProto);
gc_msg!(tf_gcmessages::CTFLobbyInviteProto);
gc_msg!(tf_gcmessages::CSOTFGameServerLobby);
gc_msg!(tf_gcmessages::CMsgExitMatchmaking);
gc_msg!(tf_gcmessages::CMsgAcceptLobbyInvite);
gc_msg!(tf_gcmessages::CMsgAcceptLobbyInviteReply);
gc_msg!(tf_gcmessages::CMsgMatchmakingSearchCountRequest);
gc_msg!(tf_gcmessages::CMsgMatchmakingSearchCountResponse);
gc_msg!(tf_gcmessages::CMsgKickedFromMatchmakingQueue);
gc_msg!(tf_gcmessages::CMsgGameServerMatchmakingStatus);
gc_msg!(tf_gcmessages::CMsgGameServerMatchmakingStatus_Player);
gc_msg!(tf_gcmessages::CMsgMatchmakingProgress);
gc_msg!(tf_gcmessages::CMsgMvMVictoryInfo);
gc_msg!(tf_gcmessages::CMsgMvMVictoryInfo_Item);
gc_msg!(tf_gcmessages::CMsgMvMVictoryInfo_Player);
gc_msg!(tf_gcmessages::CGCMsgTFHelloResponse);
gc_msg!(tf_gcmessages::CGCMsgTFSync);
gc_msg!(tf_gcmessages::CGCMsgTFSyncEx);
gc_msg!(tf_gcmessages::CMsgMvMVictory);
gc_msg!(tf_gcmessages::CMsgMvMVictory_Player);
gc_msg!(tf_gcmessages::CMsgMvMMannUpVictoryReply);
gc_msg!(tf_gcmessages::CMsgGameServerKickingLobby);
gc_msg!(tf_gcmessages::CMsgGameServerKickingLobbyResponse);
gc_msg!(tf_gcmessages::CMsgLeaveGameAndPrepareToJoinParty);
gc_msg!(tf_gcmessages::CMsgPlayerLeftMatch);
gc_msg!(tf_gcmessages::CMsgPlayerLeftMatchResponse);
gc_msg!(tf_gcmessages::CMsgPlayerVoteKickedAfterLeavingMatch);
gc_msg!(tf_gcmessages::CMsgPlayerVoteKickedAfterLeavingMatchResponse);
gc_msg!(tf_gcmessages::CMsgHalloween_ServerBossEvent);
gc_msg!(tf_gcmessages::CMsgHalloween_Merasmus2012);
gc_msg!(tf_gcmessages::CMsgUpdateHalloweenMerasmusLootLevel);
gc_msg!(tf_gcmessages::CMsgUpdateHalloweenMerasmusLootLevel_Player);
gc_msg!(tf_gcmessages::CAttribute_String);
gc_msg!(tf_gcmessages::CAttribute_DynamicRecipeComponent);
gc_msg!(tf_gcmessages::CAttribute_DynamicRecipeComponent_COMPAT_NEVER_SERIALIZE_THIS_OUT);
gc_msg!(tf_gcmessages::CAttribute_ItemSlotCriteria);
gc_msg!(tf_gcmessages::CMsgSetItemSlotAttribute);
gc_msg!(tf_gcmessages::CSOWarData);
gc_msg!(tf_gcmessages::CGCMsgGC_War_IndividualUpdate);
gc_msg!(tf_gcmessages::CGCMsgGC_War_JoinWar);
gc_msg!(tf_gcmessages::CGCMsgGC_War_RequestGlobalStats);
gc_msg!(tf_gcmessages::CGCMsgGC_War_GlobalStatsResponse);
gc_msg!(tf_gcmessages::CGCMsgGC_War_GlobalStatsResponse_SideScore);
gc_msg!(tf_gcmessages::CGCMsgGC_PlayerDuckLeaderboard_IndividualUpdate);
gc_msg!(tf_gcmessages::CAttribute_WorldItemPlacement);
gc_msg!(tf_gcmessages::CGCMsg_WorldItemPlacement_Update);
gc_msg!(tf_gcmessages::CMsgAcknowledgeXP);
gc_msg!(tf_gcmessages::CMsgTFXPSource);
gc_msg!(tf_gcmessages::CMsgTFXPSourceBreakdown);
gc_msg!(tf_gcmessages::CMsgTFClientInit);
gc_msg!(tf_gcmessages::CMsgGCNotification);
gc_msg!(tf_gcmessages::CMsgGCNotificationQueue);
gc_msg!(tf_gcmessages::CMsgNotificationAcknowledge);
gc_msg!(tf_gcmessages::CMsgNotificationAcknowledgeReply);
gc_msg!(tf_gcmessages::CMsgGC_Match_Result);
gc_msg!(tf_gcmessages::CMsgGC_Match_Result_Player);
gc_msg!(tf_gcmessages::CMsgGC_Match_ResultResponse);
gc_msg!(tf_gcmessages::CEconItemPreviewDataBlock);
gc_msg!(tf_gcmessages::CMsgGC_Client2GCEconPreviewDataBlockRequest);
gc_msg!(tf_gcmessages::CMsgGC_Client2GCEconPreviewDataBlockResponse);
gc_msg!(tf_gcmessages::CSOTFLadderPlayerStats);
gc_msg!(tf_gcmessages::CSOTFRatingData);
gc_msg!(tf_gcmessages::CMsgGC_TFVoteKickPlayerRequest);
gc_msg!(tf_gcmessages::CMsgGC_VoteKickPlayerRequestResponse);
gc_msg!(tf_gcmessages::CMsgGC_DailyCompetitiveStatsRollup);
gc_msg!(tf_gcmessages::CMsgGC_DailyCompetitiveStatsRollup_Response);
gc_msg!(tf_gcmessages::CMsgGC_DailyCompetitiveStatsRollup_Response_RankBucketEntry);
gc_msg!(tf_gcmessages::CMsgGC_ReportPlayer);
gc_msg!(tf_gcmessages::CSOTFMatchResultPlayerStats);
gc_msg!(tf_gcmessages::CMsgGCRequestMatchMakerStats);
gc_msg!(tf_gcmessages::CMsgGCDataCenterPopulation);
gc_msg!(tf_gcmessages::CMsgGCMatchGroupDataCenterPopulation);
gc_msg!(tf_gcmessages::CMsgGCMatchMakerStatsResponse);
gc_msg!(tf_gcmessages::CMsgGCMatchHistoryLoad);
gc_msg!(tf_gcmessages::CMsgGCDataCenterPing_Update);
gc_msg!(tf_gcmessages::CMsgGCDataCenterPing_Update_PingEntry);
gc_msg!(tf_gcmessages::CMsgGC_KickPlayerFromLobby);
gc_msg!(tf_gcmessages::CMsgGCSurveyRequest);
gc_msg!(tf_gcmessages::CMsgGCSurveyResponse);
gc_msg!(tf_gcmessages::CSOQuestMapNode);
gc_msg!(tf_gcmessages::CSOQuest);
gc_msg!(tf_gcmessages::CSOQuestMapRewardPurchase);
gc_msg!(tf_gcmessages::CMsgGCQuestIdentify);
gc_msg!(tf_gcmessages::CMsgGCQuestDevGive);
gc_msg!(tf_gcmessages::CMsgGCQuestNodeTurnIn);
gc_msg!(tf_gcmessages::CMsgGCQuestMapUnlockNode);
gc_msg!(tf_gcmessages::CMsgGCNewMatchForLobbyRequest);
gc_msg!(tf_gcmessages::CMsgGCNewMatchForLobbyResponse);
gc_msg!(tf_gcmessages::CMsgGCChangeMatchPlayerTeamsRequest);
gc_msg!(tf_gcmessages::CMsgGCChangeMatchPlayerTeamsRequest_Member);
gc_msg!(tf_gcmessages::CMsgGCChangeMatchPlayerTeamsResponse);
gc_msg!(tf_gcmessages::CMsgGCQuestComplete_Debug);
gc_msg!(tf_gcmessages::CMsgGCQuestMap_Debug);
gc_msg!(tf_gcmessages::CMsgGCQuestMapPurchaseReward);
gc_msg!(tf_gcmessages::CMsgGCQuestResponse);
gc_msg!(tf_gcmessages::CMsgGCSetDisablePartyQuestProgress);
gc_msg!(tf_gcmessages::CMsgQuestProgressReport);
gc_msg!(tf_gcmessages::CMsgConsumePaintkit);
gc_msg!(tf_gcmessages::CMsgPainkitDevGrant);
gc_msg!(tf_gcmessages::GCQuestStrangeEvent);

