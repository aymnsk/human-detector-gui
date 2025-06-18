import streamlit as st
import os
import tempfile
from human_detector import detect_humans

st.title("Human Detector")
st.write("Upload a video to detect humans")

uploaded_file = st.file_uploader("Choose a video...", type=["mp4", "avi", "mov"])

if uploaded_file is not None:
    # Save uploaded file to temp location
    with tempfile.NamedTemporaryFile(delete=False, suffix=".mp4") as tmp_input:
        tmp_input.write(uploaded_file.read())
        input_path = tmp_input.name
    
    # Prepare output file
    output_path = os.path.join(tempfile.gettempdir(), "output.mp4")
    
    if st.button("Process Video"):
        try:
            with st.spinner("Processing video..."):
                detect_humans(input_path, output_path)
            
            st.success("Processing complete!")
            
            # Show download button
            with open(output_path, "rb") as f:
                st.download_button(
                    label="Download Processed Video",
                    data=f,
                    file_name="processed_video.mp4",
                    mime="video/mp4"
                )
        except Exception as e:
            st.error(f"Error processing video: {e}")
        
        # Clean up
        os.unlink(input_path)
        if os.path.exists(output_path):
            os.unlink(output_path)
