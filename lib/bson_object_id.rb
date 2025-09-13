# frozen_string_literal: true

require_relative "bson_object_id/version"
require_relative "bson_object_id/bson_object_id"

module BSON

  class ObjectId
    include Comparable
  end

  class Error < StandardError; end
  # Your code goes here...
end
