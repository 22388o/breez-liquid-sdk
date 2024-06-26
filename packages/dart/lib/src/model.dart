// This file is automatically generated, so please do not edit it.
// Generated by `flutter_rust_bridge`@ 2.0.0-dev.38.

// ignore_for_file: invalid_use_of_internal_member, unused_import, unnecessary_import

import 'frb_generated.dart';
import 'package:flutter_rust_bridge/flutter_rust_bridge_for_generated.dart';
import 'package:freezed_annotation/freezed_annotation.dart' hide protected;
part 'model.freezed.dart';

class BackupRequest {
  /// Path to the backup.
  ///
  /// If not set, it defaults to `backup.sql` for mainnet and `backup-testnet.sql` for testnet.
  /// The file will be saved in [ConnectRequest]'s `data_dir`.
  final String? backupPath;

  const BackupRequest({
    this.backupPath,
  });

  @override
  int get hashCode => backupPath.hashCode;

  @override
  bool operator ==(Object other) =>
      identical(this, other) ||
      other is BackupRequest && runtimeType == other.runtimeType && backupPath == other.backupPath;
}

/// Configuration for the Liquid SDK
class Config {
  final String boltzUrl;
  final String electrumUrl;

  /// Directory in which all SDK files (DB, log, cache) are stored.
  ///
  /// Prefix can be a relative or absolute path to this directory.
  final String workingDir;
  final Network network;

  /// Send payment timeout. See [crate::sdk::LiquidSdk::send_payment]
  final BigInt paymentTimeoutSec;

  /// Zero-conf minimum accepted fee-rate in sat/vbyte
  final double zeroConfMinFeeRate;

  /// Maximum amount in satoshi to accept zero-conf payments with
  /// Defaults to [crate::receive_swap::DEFAULT_ZERO_CONF_MAX_SAT]
  final BigInt? zeroConfMaxAmountSat;

  const Config({
    required this.boltzUrl,
    required this.electrumUrl,
    required this.workingDir,
    required this.network,
    required this.paymentTimeoutSec,
    required this.zeroConfMinFeeRate,
    this.zeroConfMaxAmountSat,
  });

  @override
  int get hashCode =>
      boltzUrl.hashCode ^
      electrumUrl.hashCode ^
      workingDir.hashCode ^
      network.hashCode ^
      paymentTimeoutSec.hashCode ^
      zeroConfMinFeeRate.hashCode ^
      zeroConfMaxAmountSat.hashCode;

  @override
  bool operator ==(Object other) =>
      identical(this, other) ||
      other is Config &&
          runtimeType == other.runtimeType &&
          boltzUrl == other.boltzUrl &&
          electrumUrl == other.electrumUrl &&
          workingDir == other.workingDir &&
          network == other.network &&
          paymentTimeoutSec == other.paymentTimeoutSec &&
          zeroConfMinFeeRate == other.zeroConfMinFeeRate &&
          zeroConfMaxAmountSat == other.zeroConfMaxAmountSat;
}

class ConnectRequest {
  final String mnemonic;
  final Config config;

  const ConnectRequest({
    required this.mnemonic,
    required this.config,
  });

  @override
  int get hashCode => mnemonic.hashCode ^ config.hashCode;

  @override
  bool operator ==(Object other) =>
      identical(this, other) ||
      other is ConnectRequest &&
          runtimeType == other.runtimeType &&
          mnemonic == other.mnemonic &&
          config == other.config;
}

class GetInfoResponse {
  /// Usable balance. This is the confirmed onchain balance minus `pending_send_sat`.
  final BigInt balanceSat;

  /// Amount that is being used for ongoing Send swaps
  final BigInt pendingSendSat;

  /// Incoming amount that is pending from ongoing Receive swaps
  final BigInt pendingReceiveSat;
  final String pubkey;

  const GetInfoResponse({
    required this.balanceSat,
    required this.pendingSendSat,
    required this.pendingReceiveSat,
    required this.pubkey,
  });

  @override
  int get hashCode =>
      balanceSat.hashCode ^ pendingSendSat.hashCode ^ pendingReceiveSat.hashCode ^ pubkey.hashCode;

  @override
  bool operator ==(Object other) =>
      identical(this, other) ||
      other is GetInfoResponse &&
          runtimeType == other.runtimeType &&
          balanceSat == other.balanceSat &&
          pendingSendSat == other.pendingSendSat &&
          pendingReceiveSat == other.pendingReceiveSat &&
          pubkey == other.pubkey;
}

@freezed
sealed class LiquidSdkEvent with _$LiquidSdkEvent {
  const LiquidSdkEvent._();

  const factory LiquidSdkEvent.paymentFailed({
    required Payment details,
  }) = LiquidSdkEvent_PaymentFailed;
  const factory LiquidSdkEvent.paymentPending({
    required Payment details,
  }) = LiquidSdkEvent_PaymentPending;
  const factory LiquidSdkEvent.paymentRefunded({
    required Payment details,
  }) = LiquidSdkEvent_PaymentRefunded;
  const factory LiquidSdkEvent.paymentRefundPending({
    required Payment details,
  }) = LiquidSdkEvent_PaymentRefundPending;
  const factory LiquidSdkEvent.paymentSucceeded({
    required Payment details,
  }) = LiquidSdkEvent_PaymentSucceeded;
  const factory LiquidSdkEvent.paymentWaitingConfirmation({
    required Payment details,
  }) = LiquidSdkEvent_PaymentWaitingConfirmation;
  const factory LiquidSdkEvent.synced() = LiquidSdkEvent_Synced;
}

/// Wrapper for a BOLT11 LN invoice
class LNInvoice {
  final String bolt11;
  final Network network;
  final String payeePubkey;
  final String paymentHash;
  final String? description;
  final String? descriptionHash;
  final BigInt? amountMsat;
  final BigInt timestamp;
  final BigInt expiry;
  final List<RouteHint> routingHints;
  final Uint8List paymentSecret;
  final BigInt minFinalCltvExpiryDelta;

  const LNInvoice({
    required this.bolt11,
    required this.network,
    required this.payeePubkey,
    required this.paymentHash,
    this.description,
    this.descriptionHash,
    this.amountMsat,
    required this.timestamp,
    required this.expiry,
    required this.routingHints,
    required this.paymentSecret,
    required this.minFinalCltvExpiryDelta,
  });

  @override
  int get hashCode =>
      bolt11.hashCode ^
      network.hashCode ^
      payeePubkey.hashCode ^
      paymentHash.hashCode ^
      description.hashCode ^
      descriptionHash.hashCode ^
      amountMsat.hashCode ^
      timestamp.hashCode ^
      expiry.hashCode ^
      routingHints.hashCode ^
      paymentSecret.hashCode ^
      minFinalCltvExpiryDelta.hashCode;

  @override
  bool operator ==(Object other) =>
      identical(this, other) ||
      other is LNInvoice &&
          runtimeType == other.runtimeType &&
          bolt11 == other.bolt11 &&
          network == other.network &&
          payeePubkey == other.payeePubkey &&
          paymentHash == other.paymentHash &&
          description == other.description &&
          descriptionHash == other.descriptionHash &&
          amountMsat == other.amountMsat &&
          timestamp == other.timestamp &&
          expiry == other.expiry &&
          routingHints == other.routingHints &&
          paymentSecret == other.paymentSecret &&
          minFinalCltvExpiryDelta == other.minFinalCltvExpiryDelta;
}

/// Internal SDK log entry used in the Uniffi and Dart bindings
class LogEntry {
  final String line;
  final String level;

  const LogEntry({
    required this.line,
    required this.level,
  });

  @override
  int get hashCode => line.hashCode ^ level.hashCode;

  @override
  bool operator ==(Object other) =>
      identical(this, other) ||
      other is LogEntry && runtimeType == other.runtimeType && line == other.line && level == other.level;
}

enum Network {
  /// Mainnet Bitcoin and Liquid chains
  mainnet,

  /// Testnet Bitcoin and Liquid chains
  testnet,
  ;
}

/// Represents an SDK payment.
///
/// By default, this is an onchain tx. It may represent a swap, if swap metadata is available.
class Payment {
  final String? txId;

  /// The swap ID, if any swap is associated with this payment
  final String? swapId;

  /// Composite timestamp that can be used for sorting or displaying the payment.
  ///
  /// If this payment has an associated swap, it is the swap creation time. Otherwise, the point
  /// in time when the underlying tx was included in a block. If there is no associated swap
  /// available and the underlying tx is not yet confirmed, the value is `now()`.
  final int timestamp;

  /// The payment amount, which corresponds to the onchain tx amount.
  ///
  /// In case of an outbound payment (Send), this is the payer amount. Otherwise it's the receiver amount.
  final BigInt amountSat;

  /// Represents the fees paid by this wallet for this payment.
  ///
  /// ### Swaps
  /// If there is an associated Send Swap, these fees represent the total fees paid by this wallet
  /// (the sender). It is the difference between the amount that was sent and the amount received.
  ///
  /// If there is an associated Receive Swap, these fees represent the total fees paid by this wallet
  /// (the receiver). It is also the difference between the amount that was sent and the amount received.
  ///
  /// ### Pure onchain txs
  /// If no swap is associated with this payment:
  /// - for Send payments, this is the onchain tx fee
  /// - for Receive payments, this is zero
  final BigInt feesSat;

  /// In case of a Send swap, this is the preimage of the paid invoice (proof of payment).
  final String? preimage;

  /// For a Send swap which was refunded, this is the refund tx id
  final String? refundTxId;

  /// For a Send swap which was refunded, this is the refund amount
  final BigInt? refundTxAmountSat;
  final PaymentType paymentType;

  /// Composite status representing the overall status of the payment.
  ///
  /// If the tx has no associated swap, this reflects the onchain tx status (confirmed or not).
  ///
  /// If the tx has an associated swap, this is determined by the swap status (pending or complete).
  final PaymentState status;

  const Payment({
    this.txId,
    this.swapId,
    required this.timestamp,
    required this.amountSat,
    required this.feesSat,
    this.preimage,
    this.refundTxId,
    this.refundTxAmountSat,
    required this.paymentType,
    required this.status,
  });

  @override
  int get hashCode =>
      txId.hashCode ^
      swapId.hashCode ^
      timestamp.hashCode ^
      amountSat.hashCode ^
      feesSat.hashCode ^
      preimage.hashCode ^
      refundTxId.hashCode ^
      refundTxAmountSat.hashCode ^
      paymentType.hashCode ^
      status.hashCode;

  @override
  bool operator ==(Object other) =>
      identical(this, other) ||
      other is Payment &&
          runtimeType == other.runtimeType &&
          txId == other.txId &&
          swapId == other.swapId &&
          timestamp == other.timestamp &&
          amountSat == other.amountSat &&
          feesSat == other.feesSat &&
          preimage == other.preimage &&
          refundTxId == other.refundTxId &&
          refundTxAmountSat == other.refundTxAmountSat &&
          paymentType == other.paymentType &&
          status == other.status;
}

enum PaymentState {
  created,

  /// ## Receive Swaps
  ///
  /// Covers the cases when
  /// - the lockup tx is seen in the mempool or
  /// - our claim tx is broadcast
  ///
  /// When the claim tx is broadcast, `claim_tx_id` is set in the swap.
  ///
  /// ## Send Swaps
  ///
  /// Covers the cases when
  /// - our lockup tx was broadcast or
  /// - a refund was initiated and our refund tx was broadcast
  ///
  /// When the refund tx is broadcast, `refund_tx_id` is set in the swap.
  ///
  /// ## No swap data available
  ///
  /// If no associated swap is found, this indicates the underlying tx is not confirmed yet.
  pending,

  /// ## Receive Swaps
  ///
  /// Covers the case when the claim tx is confirmed.
  ///
  /// ## Send Swaps
  ///
  /// This is the status when the claim tx is broadcast and we see it in the mempool.
  ///
  /// ## No swap data available
  ///
  /// If no associated swap is found, this indicates the underlying tx is confirmed.
  complete,

  /// ## Receive Swaps
  ///
  /// This is the status when the swap failed for any reason and the Receive could not complete.
  ///
  /// ## Send Swaps
  ///
  /// This is the status when a swap refund was initiated and the refund tx is confirmed.
  failed,

  /// ## Send Swaps
  ///
  /// This covers the case when the swap state is still Created and the swap fails to reach the
  /// Pending state in time. The TimedOut state indicates the lockup tx should never be broadcast.
  timedOut,
  ;
}

enum PaymentType {
  receive,
  send,
  ;
}

class PrepareReceiveRequest {
  final BigInt payerAmountSat;

  const PrepareReceiveRequest({
    required this.payerAmountSat,
  });

  @override
  int get hashCode => payerAmountSat.hashCode;

  @override
  bool operator ==(Object other) =>
      identical(this, other) ||
      other is PrepareReceiveRequest &&
          runtimeType == other.runtimeType &&
          payerAmountSat == other.payerAmountSat;
}

class PrepareReceiveResponse {
  final BigInt payerAmountSat;
  final BigInt feesSat;

  const PrepareReceiveResponse({
    required this.payerAmountSat,
    required this.feesSat,
  });

  @override
  int get hashCode => payerAmountSat.hashCode ^ feesSat.hashCode;

  @override
  bool operator ==(Object other) =>
      identical(this, other) ||
      other is PrepareReceiveResponse &&
          runtimeType == other.runtimeType &&
          payerAmountSat == other.payerAmountSat &&
          feesSat == other.feesSat;
}

class PrepareSendRequest {
  final String invoice;

  const PrepareSendRequest({
    required this.invoice,
  });

  @override
  int get hashCode => invoice.hashCode;

  @override
  bool operator ==(Object other) =>
      identical(this, other) ||
      other is PrepareSendRequest && runtimeType == other.runtimeType && invoice == other.invoice;
}

class PrepareSendResponse {
  final String invoice;
  final BigInt feesSat;

  const PrepareSendResponse({
    required this.invoice,
    required this.feesSat,
  });

  @override
  int get hashCode => invoice.hashCode ^ feesSat.hashCode;

  @override
  bool operator ==(Object other) =>
      identical(this, other) ||
      other is PrepareSendResponse &&
          runtimeType == other.runtimeType &&
          invoice == other.invoice &&
          feesSat == other.feesSat;
}

class ReceivePaymentResponse {
  final String id;
  final String invoice;

  const ReceivePaymentResponse({
    required this.id,
    required this.invoice,
  });

  @override
  int get hashCode => id.hashCode ^ invoice.hashCode;

  @override
  bool operator ==(Object other) =>
      identical(this, other) ||
      other is ReceivePaymentResponse &&
          runtimeType == other.runtimeType &&
          id == other.id &&
          invoice == other.invoice;
}

class RestoreRequest {
  final String? backupPath;

  const RestoreRequest({
    this.backupPath,
  });

  @override
  int get hashCode => backupPath.hashCode;

  @override
  bool operator ==(Object other) =>
      identical(this, other) ||
      other is RestoreRequest && runtimeType == other.runtimeType && backupPath == other.backupPath;
}

/// A route hint for a LN payment
class RouteHint {
  final List<RouteHintHop> hops;

  const RouteHint({
    required this.hops,
  });

  @override
  int get hashCode => hops.hashCode;

  @override
  bool operator ==(Object other) =>
      identical(this, other) || other is RouteHint && runtimeType == other.runtimeType && hops == other.hops;
}

/// Details of a specific hop in a larger route hint
class RouteHintHop {
  /// The node_id of the non-target end of the route
  final String srcNodeId;

  /// The short_channel_id of this channel
  final BigInt shortChannelId;

  /// The fees which must be paid to use this channel
  final int feesBaseMsat;
  final int feesProportionalMillionths;

  /// The difference in CLTV values between this node and the next node.
  final BigInt cltvExpiryDelta;

  /// The minimum value, in msat, which must be relayed to the next hop.
  final BigInt? htlcMinimumMsat;

  /// The maximum value in msat available for routing with a single HTLC.
  final BigInt? htlcMaximumMsat;

  const RouteHintHop({
    required this.srcNodeId,
    required this.shortChannelId,
    required this.feesBaseMsat,
    required this.feesProportionalMillionths,
    required this.cltvExpiryDelta,
    this.htlcMinimumMsat,
    this.htlcMaximumMsat,
  });

  @override
  int get hashCode =>
      srcNodeId.hashCode ^
      shortChannelId.hashCode ^
      feesBaseMsat.hashCode ^
      feesProportionalMillionths.hashCode ^
      cltvExpiryDelta.hashCode ^
      htlcMinimumMsat.hashCode ^
      htlcMaximumMsat.hashCode;

  @override
  bool operator ==(Object other) =>
      identical(this, other) ||
      other is RouteHintHop &&
          runtimeType == other.runtimeType &&
          srcNodeId == other.srcNodeId &&
          shortChannelId == other.shortChannelId &&
          feesBaseMsat == other.feesBaseMsat &&
          feesProportionalMillionths == other.feesProportionalMillionths &&
          cltvExpiryDelta == other.cltvExpiryDelta &&
          htlcMinimumMsat == other.htlcMinimumMsat &&
          htlcMaximumMsat == other.htlcMaximumMsat;
}

class SendPaymentResponse {
  final Payment payment;

  const SendPaymentResponse({
    required this.payment,
  });

  @override
  int get hashCode => payment.hashCode;

  @override
  bool operator ==(Object other) =>
      identical(this, other) ||
      other is SendPaymentResponse && runtimeType == other.runtimeType && payment == other.payment;
}
