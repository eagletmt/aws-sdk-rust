// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
/// <p>Information about a model deployed on an edge device that is registered with SageMaker Edge Manager.</p>
#[non_exhaustive]
#[derive(std::clone::Clone, std::cmp::PartialEq)]
pub struct Model {
    /// <p>The name of the model.</p>
    pub model_name: std::option::Option<std::string::String>,
    /// <p>The version of the model.</p>
    pub model_version: std::option::Option<std::string::String>,
    /// <p>The timestamp of the last data sample taken.</p>
    pub latest_sample_time: std::option::Option<aws_smithy_types::DateTime>,
    /// <p>The timestamp of the last inference that was made.</p>
    pub latest_inference: std::option::Option<aws_smithy_types::DateTime>,
    /// <p>Information required for model metrics.</p>
    pub model_metrics: std::option::Option<std::vec::Vec<crate::model::EdgeMetric>>,
}
impl Model {
    /// <p>The name of the model.</p>
    pub fn model_name(&self) -> std::option::Option<&str> {
        self.model_name.as_deref()
    }
    /// <p>The version of the model.</p>
    pub fn model_version(&self) -> std::option::Option<&str> {
        self.model_version.as_deref()
    }
    /// <p>The timestamp of the last data sample taken.</p>
    pub fn latest_sample_time(&self) -> std::option::Option<&aws_smithy_types::DateTime> {
        self.latest_sample_time.as_ref()
    }
    /// <p>The timestamp of the last inference that was made.</p>
    pub fn latest_inference(&self) -> std::option::Option<&aws_smithy_types::DateTime> {
        self.latest_inference.as_ref()
    }
    /// <p>Information required for model metrics.</p>
    pub fn model_metrics(&self) -> std::option::Option<&[crate::model::EdgeMetric]> {
        self.model_metrics.as_deref()
    }
}
impl std::fmt::Debug for Model {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut formatter = f.debug_struct("Model");
        formatter.field("model_name", &self.model_name);
        formatter.field("model_version", &self.model_version);
        formatter.field("latest_sample_time", &self.latest_sample_time);
        formatter.field("latest_inference", &self.latest_inference);
        formatter.field("model_metrics", &self.model_metrics);
        formatter.finish()
    }
}
/// See [`Model`](crate::model::Model)
pub mod model {
    /// A builder for [`Model`](crate::model::Model)
    #[non_exhaustive]
    #[derive(std::default::Default, std::clone::Clone, std::cmp::PartialEq, std::fmt::Debug)]
    pub struct Builder {
        pub(crate) model_name: std::option::Option<std::string::String>,
        pub(crate) model_version: std::option::Option<std::string::String>,
        pub(crate) latest_sample_time: std::option::Option<aws_smithy_types::DateTime>,
        pub(crate) latest_inference: std::option::Option<aws_smithy_types::DateTime>,
        pub(crate) model_metrics: std::option::Option<std::vec::Vec<crate::model::EdgeMetric>>,
    }
    impl Builder {
        /// <p>The name of the model.</p>
        pub fn model_name(mut self, input: impl Into<std::string::String>) -> Self {
            self.model_name = Some(input.into());
            self
        }
        /// <p>The name of the model.</p>
        pub fn set_model_name(mut self, input: std::option::Option<std::string::String>) -> Self {
            self.model_name = input;
            self
        }
        /// <p>The version of the model.</p>
        pub fn model_version(mut self, input: impl Into<std::string::String>) -> Self {
            self.model_version = Some(input.into());
            self
        }
        /// <p>The version of the model.</p>
        pub fn set_model_version(
            mut self,
            input: std::option::Option<std::string::String>,
        ) -> Self {
            self.model_version = input;
            self
        }
        /// <p>The timestamp of the last data sample taken.</p>
        pub fn latest_sample_time(mut self, input: aws_smithy_types::DateTime) -> Self {
            self.latest_sample_time = Some(input);
            self
        }
        /// <p>The timestamp of the last data sample taken.</p>
        pub fn set_latest_sample_time(
            mut self,
            input: std::option::Option<aws_smithy_types::DateTime>,
        ) -> Self {
            self.latest_sample_time = input;
            self
        }
        /// <p>The timestamp of the last inference that was made.</p>
        pub fn latest_inference(mut self, input: aws_smithy_types::DateTime) -> Self {
            self.latest_inference = Some(input);
            self
        }
        /// <p>The timestamp of the last inference that was made.</p>
        pub fn set_latest_inference(
            mut self,
            input: std::option::Option<aws_smithy_types::DateTime>,
        ) -> Self {
            self.latest_inference = input;
            self
        }
        /// Appends an item to `model_metrics`.
        ///
        /// To override the contents of this collection use [`set_model_metrics`](Self::set_model_metrics).
        ///
        /// <p>Information required for model metrics.</p>
        pub fn model_metrics(mut self, input: impl Into<crate::model::EdgeMetric>) -> Self {
            let mut v = self.model_metrics.unwrap_or_default();
            v.push(input.into());
            self.model_metrics = Some(v);
            self
        }
        /// <p>Information required for model metrics.</p>
        pub fn set_model_metrics(
            mut self,
            input: std::option::Option<std::vec::Vec<crate::model::EdgeMetric>>,
        ) -> Self {
            self.model_metrics = input;
            self
        }
        /// Consumes the builder and constructs a [`Model`](crate::model::Model)
        pub fn build(self) -> crate::model::Model {
            crate::model::Model {
                model_name: self.model_name,
                model_version: self.model_version,
                latest_sample_time: self.latest_sample_time,
                latest_inference: self.latest_inference,
                model_metrics: self.model_metrics,
            }
        }
    }
}
impl Model {
    /// Creates a new builder-style object to manufacture [`Model`](crate::model::Model)
    pub fn builder() -> crate::model::model::Builder {
        crate::model::model::Builder::default()
    }
}

/// <p>Information required for edge device metrics.</p>
#[non_exhaustive]
#[derive(std::clone::Clone, std::cmp::PartialEq)]
pub struct EdgeMetric {
    /// <p>The dimension of metrics published.</p>
    pub dimension: std::option::Option<std::string::String>,
    /// <p>Returns the name of the metric.</p>
    pub metric_name: std::option::Option<std::string::String>,
    /// <p>Returns the value of the metric.</p>
    pub value: f64,
    /// <p>Timestamp of when the metric was requested.</p>
    pub timestamp: std::option::Option<aws_smithy_types::DateTime>,
}
impl EdgeMetric {
    /// <p>The dimension of metrics published.</p>
    pub fn dimension(&self) -> std::option::Option<&str> {
        self.dimension.as_deref()
    }
    /// <p>Returns the name of the metric.</p>
    pub fn metric_name(&self) -> std::option::Option<&str> {
        self.metric_name.as_deref()
    }
    /// <p>Returns the value of the metric.</p>
    pub fn value(&self) -> f64 {
        self.value
    }
    /// <p>Timestamp of when the metric was requested.</p>
    pub fn timestamp(&self) -> std::option::Option<&aws_smithy_types::DateTime> {
        self.timestamp.as_ref()
    }
}
impl std::fmt::Debug for EdgeMetric {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut formatter = f.debug_struct("EdgeMetric");
        formatter.field("dimension", &self.dimension);
        formatter.field("metric_name", &self.metric_name);
        formatter.field("value", &self.value);
        formatter.field("timestamp", &self.timestamp);
        formatter.finish()
    }
}
/// See [`EdgeMetric`](crate::model::EdgeMetric)
pub mod edge_metric {
    /// A builder for [`EdgeMetric`](crate::model::EdgeMetric)
    #[non_exhaustive]
    #[derive(std::default::Default, std::clone::Clone, std::cmp::PartialEq, std::fmt::Debug)]
    pub struct Builder {
        pub(crate) dimension: std::option::Option<std::string::String>,
        pub(crate) metric_name: std::option::Option<std::string::String>,
        pub(crate) value: std::option::Option<f64>,
        pub(crate) timestamp: std::option::Option<aws_smithy_types::DateTime>,
    }
    impl Builder {
        /// <p>The dimension of metrics published.</p>
        pub fn dimension(mut self, input: impl Into<std::string::String>) -> Self {
            self.dimension = Some(input.into());
            self
        }
        /// <p>The dimension of metrics published.</p>
        pub fn set_dimension(mut self, input: std::option::Option<std::string::String>) -> Self {
            self.dimension = input;
            self
        }
        /// <p>Returns the name of the metric.</p>
        pub fn metric_name(mut self, input: impl Into<std::string::String>) -> Self {
            self.metric_name = Some(input.into());
            self
        }
        /// <p>Returns the name of the metric.</p>
        pub fn set_metric_name(mut self, input: std::option::Option<std::string::String>) -> Self {
            self.metric_name = input;
            self
        }
        /// <p>Returns the value of the metric.</p>
        pub fn value(mut self, input: f64) -> Self {
            self.value = Some(input);
            self
        }
        /// <p>Returns the value of the metric.</p>
        pub fn set_value(mut self, input: std::option::Option<f64>) -> Self {
            self.value = input;
            self
        }
        /// <p>Timestamp of when the metric was requested.</p>
        pub fn timestamp(mut self, input: aws_smithy_types::DateTime) -> Self {
            self.timestamp = Some(input);
            self
        }
        /// <p>Timestamp of when the metric was requested.</p>
        pub fn set_timestamp(
            mut self,
            input: std::option::Option<aws_smithy_types::DateTime>,
        ) -> Self {
            self.timestamp = input;
            self
        }
        /// Consumes the builder and constructs a [`EdgeMetric`](crate::model::EdgeMetric)
        pub fn build(self) -> crate::model::EdgeMetric {
            crate::model::EdgeMetric {
                dimension: self.dimension,
                metric_name: self.metric_name,
                value: self.value.unwrap_or_default(),
                timestamp: self.timestamp,
            }
        }
    }
}
impl EdgeMetric {
    /// Creates a new builder-style object to manufacture [`EdgeMetric`](crate::model::EdgeMetric)
    pub fn builder() -> crate::model::edge_metric::Builder {
        crate::model::edge_metric::Builder::default()
    }
}
