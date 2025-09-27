import mongoose, { Document, Schema } from 'mongoose';

export interface IVerification extends Document {
  uuid : string,
  proof_hash: string;
  created_at: Date;
  updated_at: Date;
  status : string
}

const verificationSchema = new Schema<IVerification>(
  {
    proof_hash: {
      type: String,
      required: [true, 'Proof is required'],
      unique: true,
    },
    uuid: {
      type: String,
      required: [true, 'UUID is required'],
      unique: true,
    },
    status : {
      default : "processing",
      type : String,
      enum : [ "processing", "valid" , "invalid"]
    }
  },
  {
    timestamps: true,
  }
);

// Create indexes
// verificationSchema.index({ uuid: 1 });
// verificationSchema.index({ proofHash: 1 });

export const VerificationModel = mongoose.model<IVerification>('Verification', verificationSchema);
